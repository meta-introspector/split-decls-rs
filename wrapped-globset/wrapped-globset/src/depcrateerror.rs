// Generated macro for Error (struct)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Represents an error that can occur when parsing a glob pattern."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Error { # [doc = " The original glob provided by the caller."] glob : Option < String > , # [doc = " The kind of error."] kind : ErrorKind , }
};
}
