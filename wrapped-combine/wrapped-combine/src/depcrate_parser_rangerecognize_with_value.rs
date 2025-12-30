// Generated macro for recognize_with_value (function)
macro_rules! Depcrate_parser_rangerecognize_with_value {
() => {
// Module: crate::parser::range
// Provides: {"recognize_with_value"}
// Dependencies: {}
# [doc = " Zero-copy parser which returns a pair: (committed input range, parsed value)."] # [doc = ""] # [doc = ""] # [doc = " [`combinator::recognize_with_value`] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`combinator::recognize_with_value`]: recognize_with_value"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::recognize_with_value;"] # [doc = " # use combine::parser::char::{digit, char};"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = recognize_with_value(("] # [doc = "     skip_many1(digit()),"] # [doc = "     optional((attempt(char('.')), skip_many1(digit()))),"] # [doc = " ).map(|(_, opt)| opt.is_some()));"] # [doc = ""] # [doc = " assert_eq!(parser.parse(\"1234!\"), Ok(((\"1234\", false), \"!\")));"] # [doc = " assert_eq!(parser.parse(\"1234.0001!\"), Ok(((\"1234.0001\", true), \"!\")));"] # [doc = " assert!(parser.parse(\"!\").is_err());"] # [doc = " assert!(parser.parse(\"1234.\").is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn recognize_with_value < Input , P > (parser : P) -> RecognizeWithValue < P > where P : Parser < Input > , Input : RangeStream , < Input as StreamOnce > :: Range : crate :: stream :: Range , { RecognizeWithValue (parser) }
};
}
