// Generated macro for Error (struct)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned when any config value couldn't be instantiated due to malformed input."] # [derive (Debug , thiserror :: Error , Eq , PartialEq)] # [allow (missing_docs)] # [error ("Could not decode '{input}': {message}")] pub struct Error { pub message : & 'static str , pub input : bstr :: BString , # [source] pub utf8_err : Option < std :: str :: Utf8Error > , }
};
}
