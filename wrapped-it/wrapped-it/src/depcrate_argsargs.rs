// Generated macro for Args (struct)
macro_rules! Depcrate_argsArgs {
() => {
// Module: crate::args
// Provides: {"Args"}
// Dependencies: {}
# [derive (Debug , clap :: Parser)] # [clap (name = "it" , about = "internal tools to help create test cases")] pub struct Args { # [clap (subcommand)] pub cmd : Subcommands , }
};
}
