// Generated macro for submodule (module)
macro_rules! Depcrate_plumbing_optionssubmodule {
() => {
// Module: crate::plumbing::options
// Provides: {"submodule"}
// Dependencies: {}
pub mod submodule { # [derive (Debug , clap :: Parser)] pub struct Platform { # [clap (subcommand)] pub cmds : Option < Subcommands > , } # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Print all direct submodules to standard output."] List { # [doc = " Set the suffix to append if the repository is dirty (not counting untracked files)."] # [clap (short = 'd' , long)] dirty_suffix : Option < Option < String > > , } , } }
};
}
