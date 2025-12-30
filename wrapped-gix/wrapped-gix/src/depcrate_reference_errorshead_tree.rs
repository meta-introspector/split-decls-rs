// Generated macro for head_tree (module)
macro_rules! Depcrate_reference_errorshead_tree {
() => {
// Module: crate::reference::errors
// Provides: {"head_tree"}
// Dependencies: {}
# [doc = ""] pub mod head_tree { # [doc = " The error returned by [`Repository::head_tree`(…)](crate::Repository::head_tree())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] CommitTree (# [from] crate :: object :: commit :: Error) , } }
};
}
