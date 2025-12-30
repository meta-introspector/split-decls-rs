// Generated macro for macro_377 (macro)
macro_rules! Depcrate_parser_bytemacro_377 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_377"}
// Dependencies: {}
take_until ! { # [doc = " Zero-copy parser which reads a range of 0 or more tokens until `a` is found."] # [doc = ""] # [doc = " If `a` is not found, the parser will return an error."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::byte::take_until_byte;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_until_byte(b'\\r');"] # [doc = " let result = parser.parse(\"To: user@example.com\\r\\n\");"] # [doc = " assert_eq!(result, Ok((\"To: user@example.com\", \"\\r\\n\")));"] # [doc = " let result = parser.parse(\"Hello, world\\n\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] TakeUntilByte , take_until_byte , memchr , a }
};
}
