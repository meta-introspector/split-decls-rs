// Generated macro for Error (enum)
macro_rules! Depcrate_revision_walkError {
() => {
// Module: crate::revision::walk
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Platform::all()`] and [`Platform::selected()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] SimpleTraversal (# [from] gix_traverse :: commit :: simple :: Error) , # [error (transparent)] ShallowCommits (# [from] crate :: shallow :: read :: Error) , # [error (transparent)] ConfigBoolean (# [from] crate :: config :: boolean :: Error) , }
};
}
