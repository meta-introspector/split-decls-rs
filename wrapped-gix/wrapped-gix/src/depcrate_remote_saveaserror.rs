// Generated macro for AsError (enum)
macro_rules! Depcrate_remote_saveAsError {
() => {
// Module: crate::remote::save
// Provides: {"AsError"}
// Dependencies: {}
# [doc = " The error returned by [`Remote::save_as_to()`]."] # [doc = ""] # [doc = " Note that this type should rather be in the `as` module, but cannot be as it's part of the Rust syntax."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum AsError { # [error (transparent)] Save (# [from] Error) , # [error (transparent)] Name (# [from] crate :: remote :: name :: Error) , }
};
}
