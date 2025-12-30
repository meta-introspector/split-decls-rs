// Generated macro for take_until_range (function)
macro_rules! Depcrate_parser_rangetake_until_range {
() => {
// Module: crate::parser::range
// Provides: {"take_until_range"}
// Dependencies: {}
# [doc = " Zero-copy parser which reads a range of 0 or more tokens until `r` is found."] # [doc = ""] # [doc = " The range `r` will not be committed. If `r` is not found, the parser will"] # [doc = " return an error."] # [doc = ""] # [doc = " [`repeat::take_until`][] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`repeat::take_until`]: ../../parser/repeat/fn.take_until.html"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::{range, take_until_range};"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_until_range(\"\\r\\n\");"] # [doc = " let result = parser.parse(\"To: user@example.com\\r\\n\");"] # [doc = " assert_eq!(result, Ok((\"To: user@example.com\", \"\\r\\n\")));"] # [doc = " let result = parser.parse(\"Hello, world\\n\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn take_until_range < Input > (r : Input :: Range) -> TakeUntilRange < Input > where Input : RangeStream , { TakeUntilRange (r) }
};
}
