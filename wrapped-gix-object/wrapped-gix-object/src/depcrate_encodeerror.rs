// Generated macro for Error (enum)
macro_rules! Depcrate_encodeError {
() => {
// Module: crate::encode
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error returned when object encoding fails."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Newlines are not allowed in header values: {value:?}")] NewlineInHeaderValue { value : BString } , # [error ("Header values must not be empty")] EmptyValue , }
};
}
