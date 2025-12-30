// Generated macro for iter (module)
macro_rules! Depcrate_revision_walkiter {
() => {
// Module: crate::revision::walk
// Provides: {"iter"}
// Dependencies: {}
# [doc = ""] pub mod iter { # [doc = " The error returned by the [Walk](crate::revision::Walk) iterator."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] SimpleTraversal (# [from] gix_traverse :: commit :: simple :: Error) , } }
};
}
