// Generated macro for Error (enum)
macro_rules! Depcrate_status_iterError {
() => {
// Module: crate::status::iter
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned for each item returned by [`Iter`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] IndexWorktree (# [from] index_worktree :: Error) , # [error (transparent)] TreeIndex (# [from] tree_index :: Error) , }
};
}
