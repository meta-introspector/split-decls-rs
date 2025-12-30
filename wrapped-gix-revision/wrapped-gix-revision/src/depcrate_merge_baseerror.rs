// Generated macro for Error (enum)
macro_rules! Depcrate_merge_baseError {
() => {
// Module: crate::merge_base
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by the [`merge_base()`][function::merge_base()] function."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("A commit could not be inserted into the graph")] InsertCommit (# [from] gix_revwalk :: graph :: get_or_insert_default :: Error) , }
};
}
