// Generated macro for State (struct)
macro_rules! Depcrate_walk_readdirState {
() => {
// Module: crate::walk::readdir
// Provides: {"State"}
// Dependencies: {}
pub (super) struct State { # [doc = " The entries to hold back until it's clear what to do with them."] pub on_hold : Vec < Entry > , # [doc = " The path the user is currently in, as seen from the workdir root."] worktree_relative_current_dir : Option < PathBuf > , }
};
}
