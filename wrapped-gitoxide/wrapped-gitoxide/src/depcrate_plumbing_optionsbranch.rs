// Generated macro for branch (module)
macro_rules! Depcrate_plumbing_optionsbranch {
() => {
// Module: crate::plumbing::options
// Provides: {"branch"}
// Dependencies: {}
pub mod branch { # [derive (Debug , clap :: Parser)] pub struct Platform { # [clap (subcommand)] pub cmd : Subcommands , } # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " List branches."] List { # [doc = " List remote-tracking as well as local branches."] # [clap (long , short = 'a')] all : bool , } , } }
};
}
