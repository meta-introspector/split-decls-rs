// Generated macro for range (function)
macro_rules! Depcrate_parser_rangerange {
() => {
// Module: crate::parser::range
// Provides: {"range"}
// Dependencies: {}
# [doc = " Zero-copy parser which reads a range of length `i.len()` and succeeds if `i` is equal to that"] # [doc = " range."] # [doc = ""] # [doc = " [`tokens`] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`tokens`]: super::token::tokens"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::range;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = range(\"hello\");"] # [doc = " let result = parser.parse(\"hello world\");"] # [doc = " assert_eq!(result, Ok((\"hello\", \" world\")));"] # [doc = " let result = parser.parse(\"hel world\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn range < Input > (i : Input :: Range) -> Range < Input > where Input : RangeStream , Input :: Range : PartialEq , { Range (i) }
};
}
