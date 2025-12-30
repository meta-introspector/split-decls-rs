// Generated macro for error (module)
macro_rules! Depcrate_tagerror {
() => {
// Module: crate::tag
// Provides: {"error"}
// Dependencies: {}
mod error { # [doc = " The error returned by [`tag(…)`][crate::Repository::tag()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] ReferenceNameValidation (# [from] gix_ref :: name :: Error) , # [error (transparent)] WriteObject (# [from] crate :: object :: write :: Error) , # [error (transparent)] ReferenceEdit (# [from] crate :: reference :: edit :: Error) , # [error (transparent)] DateParseError (# [from] gix_date :: parse :: Error) , } }
};
}
