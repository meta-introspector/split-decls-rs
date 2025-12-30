// Generated macro for Utf8Error (struct)
macro_rules! Depcrate_errorsUtf8Error {
() => {
// Module: crate::errors
// Provides: {"Utf8Error"}
// Dependencies: {}
# [doc = " Error returned when an invalid UTF-8 sequence is encountered."] # [doc = ""] # [doc = " See [`Utf8ErrorKind`](enum.Utf8ErrorKind.html) for the types of errors"] # [doc = " that this type can be returned for."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct Utf8Error { pub (crate) kind : Utf8ErrorKind , }
};
}
