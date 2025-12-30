// Generated macro for macro_247 (macro)
macro_rules! Depcrate_symmetric_keymacro_247 {
() => {
// Module: crate::symmetric_key
// Provides: {"macro_247"}
// Dependencies: {}
impl_str_enum ! (# [doc = " The `Encoding` type is defined in [RFC 6031 Section 3.2.7]."] # [doc = ""] # [doc = " ```text"] # [doc = "    Encoding ::= UTF8STRING (\"DECIMAL\" | \"HEXADECIMAL\" |"] # [doc = "                 \"ALPHANUMERIC\" |\"BASE64\" |\"BINARY\")"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.7]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.7"] # [derive (Copy , Clone , PartialEq , Eq)] pub enum Encoding { # [doc = " \"DECIMAL\""] Decimal => "DECIMAL" , # [doc = " \"HEXADECIMAL\","] Hexadecimal => "HEXADECIMAL" , # [doc = " \"ALPHANUMERIC\","] Alphanumeric => "ALPHANUMERIC" , # [doc = " \"BASE64\","] Base64 => "BASE64" , # [doc = " \"BINARY\","] Binary => "BINARY" , }) ;
};
}
