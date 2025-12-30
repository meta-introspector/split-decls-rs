// Generated macro for Error (struct)
macro_rules! Depcrate_remote_nameError {
() => {
// Module: crate::remote::name
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [validated()]."] # [derive (Debug , thiserror :: Error)] # [error ("remote names must be valid within refspecs for fetching: {name:?}")] # [allow (missing_docs)] pub struct Error { pub source : gix_refspec :: parse :: Error , pub name : BString , }
};
}
