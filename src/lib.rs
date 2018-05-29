extern crate console;
#[macro_use]
extern crate failure;
extern crate indicatif;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate toml;

pub mod bindgen;
pub mod build;
pub mod command;
pub mod emoji;
pub mod error;
pub mod logger;
pub mod manifest;
pub mod npm;
pub mod progressbar;
pub mod readme;

use progressbar::ProgressOutput;

lazy_static! {
    pub static ref PBAR: ProgressOutput = { ProgressOutput::new() };
}

/// 📦 ✨  pack and publish your wasm!
#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: command::Command,
    /// Log verbosity is based off the number of v used
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    pub verbosity: u8,
}
