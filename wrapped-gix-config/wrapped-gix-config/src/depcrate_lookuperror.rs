// Generated macro for Error (enum)
macro_rules! Depcrate_lookupError {
() => {
// Module: crate::lookup
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error when looking up a value, for example via [`File::try_value()`][crate::File::try_value()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error < E > { # [error (transparent)] ValueMissing (# [from] existing :: Error) , # [error (transparent)] FailedConversion (E) , }
};
}
