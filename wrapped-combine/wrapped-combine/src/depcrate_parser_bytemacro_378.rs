// Generated macro for macro_378 (macro)
macro_rules! Depcrate_parser_bytemacro_378 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_378"}
// Dependencies: {}
take_until ! { # [doc = " Zero-copy parser which reads a range of 0 or more tokens until `a` or `b` is found."] # [doc = ""] # [doc = " If `a` or `b` is not found, the parser will return an error."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::byte::take_until_byte2;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_until_byte2(b'\\r', b'\\n');"] # [doc = " let result = parser.parse(\"To: user@example.com\\r\\n\");"] # [doc = " assert_eq!(result, Ok((\"To: user@example.com\", \"\\r\\n\")));"] # [doc = " let result = parser.parse(\"Hello, world\\n\");"] # [doc = " assert_eq!(result, Ok((\"Hello, world\", \"\\n\")));"] # [doc = " # }"] # [doc = " ```"] TakeUntilByte2 , take_until_byte2 , memchr2 , a , b }
};
}
