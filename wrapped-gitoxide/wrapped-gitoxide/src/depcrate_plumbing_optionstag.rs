// Generated macro for tag (module)
macro_rules! Depcrate_plumbing_optionstag {
() => {
// Module: crate::plumbing::options
// Provides: {"tag"}
// Dependencies: {}
pub mod tag { # [derive (Debug , clap :: Parser)] pub struct Platform { # [clap (subcommand)] pub cmds : Option < Subcommands > , } # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " List all tags."] List , } }
};
}
