// Generated macro for macro_379 (macro)
macro_rules! Depcrate_parser_bytemacro_379 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_379"}
// Dependencies: {}
take_until ! { # [doc = " Zero-copy parser which reads a range of 0 or more tokens until `a`, 'b' or `c` is found."] # [doc = ""] # [doc = " If `a`, 'b' or `c` is not found, the parser will return an error."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::byte::take_until_byte3;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_until_byte3(b'\\r', b'\\n', b' ');"] # [doc = " let result = parser.parse(\"To: user@example.com\\r\\n\");"] # [doc = " assert_eq!(result, Ok((\"To:\", \" user@example.com\\r\\n\")));"] # [doc = " let result = parser.parse(\"Helloworld\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] TakeUntilByte3 , take_until_byte3 , memchr3 , a , b , c }
};
}
