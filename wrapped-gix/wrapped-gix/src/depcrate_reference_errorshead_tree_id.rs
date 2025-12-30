// Generated macro for head_tree_id (module)
macro_rules! Depcrate_reference_errorshead_tree_id {
() => {
// Module: crate::reference::errors
// Provides: {"head_tree_id"}
// Dependencies: {}
# [doc = ""] pub mod head_tree_id { # [doc = " The error returned by [`Repository::head_tree_id`(…)](crate::Repository::head_tree_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] HeadCommit (# [from] crate :: reference :: head_commit :: Error) , # [error (transparent)] DecodeCommit (# [from] gix_object :: decode :: Error) , } }
};
}
