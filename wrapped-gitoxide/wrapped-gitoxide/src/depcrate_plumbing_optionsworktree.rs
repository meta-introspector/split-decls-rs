// Generated macro for worktree (module)
macro_rules! Depcrate_plumbing_optionsworktree {
() => {
// Module: crate::plumbing::options
// Provides: {"worktree"}
// Dependencies: {}
pub mod worktree { # [derive (Debug , clap :: Parser)] # [command (about = "Commands for handling worktrees")] pub struct Platform { # [clap (subcommand)] pub cmd : SubCommands , } # [derive (Debug , clap :: Subcommand)] pub enum SubCommands { # [doc = " List all worktrees, along with some accompanying information."] List , } }
};
}
