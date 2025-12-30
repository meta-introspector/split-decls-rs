// Generated macro for head_id (module)
macro_rules! Depcrate_reference_errorshead_id {
() => {
// Module: crate::reference::errors
// Provides: {"head_id"}
// Dependencies: {}
# [doc = ""] pub mod head_id { # [doc = " The error returned by [`Repository::head_id(…)`](crate::Repository::head_id())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Head (# [from] crate :: reference :: find :: existing :: Error) , # [error (transparent)] PeelToId (# [from] crate :: head :: peel :: into_id :: Error) , } }
};
}
