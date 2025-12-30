// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { pretty_env_logger :: init () ; let args : Cli = clap :: Parser :: parse () ; let processed_cli_options = ProcessedCli :: new (args) ; match processed_cli_options . target . as_str () { "aarch64-unknown-linux-gnu" | "armv7-unknown-linux-gnueabihf" | "aarch64_be-unknown-linux-gnu" => run (ArmArchitectureTest :: create (processed_cli_options)) , "x86_64-unknown-linux-gnu" => run (X86ArchitectureTest :: create (processed_cli_options)) , _ => std :: process :: exit (0) , } }
};
}
