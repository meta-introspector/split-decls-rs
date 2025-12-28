macro_rules! deps {
    () => {
        Shells!();
        CompleteEnv!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 's , F : Fn () -> clap :: Command > CompleteEnv < 's , F > { # [doc = " Complete a [`clap::Command`]"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Builder:"] # [doc = " ```rust"] # [doc = " # use clap_complete::CompleteEnv;"] # [doc = " fn cli() -> clap::Command {"] # [doc = "     // ..."] # [doc = " #   clap::Command::new(\"empty\")"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     CompleteEnv::with_factory(cli)"] # [doc = "         .complete()"] # [doc = ""] # [doc = "     // ... rest of application logic"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Derive:"] # [doc = " ```"] # [doc = " # use clap::Parser;"] # [doc = " # use clap_complete::CompleteEnv;"] # [doc = " use clap::CommandFactory as _;"] # [doc = ""] # [doc = " #[derive(Debug, Parser)]"] # [doc = " struct Cli {"] # [doc = "     custom: Option<String>,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     CompleteEnv::with_factory(|| Cli::command())"] # [doc = "         .complete()"] # [doc = ""] # [doc = "     // ... rest of application logic"] # [doc = " }"] # [doc = " ```"] pub fn with_factory (factory : F) -> Self { Self { factory , var : "COMPLETE" , bin : None , completer : None , shells : Shells :: builtins () , } } # [doc = " Override the environment variable used for enabling completions"] pub fn var (mut self , var : & 'static str) -> Self { self . var = var ; self } # [doc = " Override the name of the binary to complete"] # [doc = ""] # [doc = " Default: `Command::get_bin_name`"] pub fn bin (mut self , bin : impl Into < String >) -> Self { self . bin = Some (bin . into ()) ; self } # [doc = " Override the binary to call to get completions"] # [doc = ""] # [doc = " Default: `args_os()[0]`"] pub fn completer (mut self , completer : impl Into < String >) -> Self { self . completer = Some (completer . into ()) ; self } # [doc = " Override the shells supported for completions"] pub fn shells (mut self , shells : Shells < 's >) -> Self { self . shells = shells ; self } }
    };
}

impl_143!();