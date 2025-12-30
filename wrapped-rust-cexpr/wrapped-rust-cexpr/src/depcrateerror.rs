// Generated macro for Error (struct)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Parsing errors specific to C parsing."] # [doc = ""] # [doc = " This is a superset of `(I, nom::ErrorKind)` that includes the additional errors specified by"] # [doc = " [`ErrorKind`]."] # [derive (Debug)] pub struct Error < I > { # [doc = " The remainder of the input stream at the time of the error."] pub input : I , # [doc = " The error that occurred."] pub error : ErrorKind , }
};
}
