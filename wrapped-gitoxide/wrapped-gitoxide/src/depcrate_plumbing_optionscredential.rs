// Generated macro for credential (module)
macro_rules! Depcrate_plumbing_optionscredential {
() => {
// Module: crate::plumbing::options
// Provides: {"credential"}
// Dependencies: {}
pub mod credential { # [derive (Debug , clap :: Subcommand)] pub enum Subcommands { # [doc = " Get the credentials fed for `url=<url>` via STDIN."] # [clap (visible_alias = "get")] Fill , # [doc = " Approve the information piped via STDIN as obtained with last call to `fill`"] # [clap (visible_alias = "store")] Approve , # [doc = " Try to resolve the given revspec and print the object names."] # [clap (visible_alias = "erase")] Reject , } }
};
}
