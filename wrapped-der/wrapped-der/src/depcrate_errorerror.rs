// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```"] # [doc = " use der::{Decode, ErrorKind, Reader};"] # [doc = ""] # [doc = " struct MyDecodable;"] # [doc = ""] # [doc = " impl<'a> Decode<'a> for MyDecodable {"] # [doc = "     type Error = der::Error;"] # [doc = ""] # [doc = "     fn decode<R: Reader<'a>>(reader: &mut R) -> Result<Self, der::Error> {"] # [doc = "         Err(reader.error(ErrorKind::OidMalformed))"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Error { # [doc = " Kind of error."] kind : ErrorKind , # [doc = " Position inside of message where error occurred."] position : Option < Length > , }
};
}
