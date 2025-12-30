// Generated macro for head_commit (module)
macro_rules! Depcrate_reference_errorshead_commit {
() => {
// Module: crate::reference::errors
// Provides: {"head_commit"}
// Dependencies: {}
# [doc = ""] pub mod head_commit { # [doc = " The error returned by [`Repository::head_commit`(…)](crate::Repository::head_commit())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Head (# [from] crate :: reference :: find :: existing :: Error) , # [error (transparent)] PeelToCommit (# [from] crate :: head :: peel :: to_commit :: Error) , } }
};
}
