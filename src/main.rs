extern crate buildchain;
extern crate clap;
extern crate serde_json;

use buildchain::{Config, Location};
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let matches = App::new("buildchain")
                    .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .takes_value(true)
                            .help("Build configuration file"))
                    .arg(Arg::with_name("output")
                            .short("o")
                            .long("output")
                            .takes_value(true)
                            .help("Build output directory"))
                    .arg(Arg::with_name("remote")
                            .short("r")
                            .long("remote")
                            .takes_value(true)
                            .help("Name of remote LXC server"))
                    .get_matches();

    let config_path = matches.value_of("config").unwrap_or("buildchain.json");
    let output_path = matches.value_of("output").unwrap_or("buildchain.out");
    let remote_opt = matches.value_of("remote");

    let mut file = match File::open(&config_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("buildchain: failed to open {}: {}", config_path, err);
            process::exit(1)
        }
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("buildchain: failed to read {}: {}", config_path, err);
            process::exit(1)
        }
    }

    let config = match serde_json::from_str::<Config>(&string) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("buildchain: failed to parse {}: {}", config_path, err);
            process::exit(1)
        }
    };

    let location = if let Some(remote) = remote_opt {
        println!("buildchain: building {} on {}", config.name, remote);
        Location::Remote(remote.to_string())
    } else {
        println!("buildchain: building {} locally", config.name);
        Location::Local
    };

    match config.run(location, output_path) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("buildchain: failed to run {}: {}", config_path, err);
            process::exit(1)
        }
    }
}
