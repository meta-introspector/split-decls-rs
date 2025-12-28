macro_rules! PathCompleter {
    () => {
        # [doc = " Complete a value as a [`std::path::Path`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use clap::Parser;"] # [doc = " use clap_complete::engine::{ArgValueCompleter, PathCompleter};"] # [doc = ""] # [doc = " #[derive(Debug, Parser)]"] # [doc = " struct Cli {"] # [doc = "     #[arg(long, add = ArgValueCompleter::new(PathCompleter::file()))]"] # [doc = "     custom: Option<String>,"] # [doc = " }"] # [doc = " ```"] pub struct PathCompleter { current_dir : Option < std :: path :: PathBuf > , # [allow (clippy :: type_complexity)] filter : Option < Box < dyn Fn (& std :: path :: Path) -> bool + Send + Sync > > , stdio : bool , }
    };
}

PathCompleter!();