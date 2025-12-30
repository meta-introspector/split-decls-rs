// Generated macro for Error (enum)
macro_rules! Depcrate_eol_convert_to_worktreeError {
() => {
// Module: crate::eol::convert_to_worktree
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error produced by [`convert_to_worktree()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not allocate buffer")] OutOfMemory (# [from] std :: collections :: TryReserveError) , }
};
}
