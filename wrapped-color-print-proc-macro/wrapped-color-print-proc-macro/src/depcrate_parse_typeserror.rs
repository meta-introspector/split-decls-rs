// Generated macro for Error (struct)
macro_rules! Depcrate_parse_typesError {
() => {
// Module: crate::parse::types
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Replacement to [`nom::error::Error`]."] # [derive (Debug , PartialEq)] pub struct Error < 'a > { pub input : Input < 'a > , pub code : ErrorKind , pub detail : Option < ErrorDetail < 'a > > , }
};
}
