// Generated macro for macro_649 (macro)
macro_rules! Depcrate_parser_repeatmacro_649 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_649"}
// Dependencies: {}
parser ! { pub struct SkipUntil ; type PartialState = < With < TakeUntil < Sink , P >, Value < Input , () >> as Parser < Input >>:: PartialState ; # [doc = " Skips input until `end` is encountered or `end` indicates that it has committed input before"] # [doc = " failing (`attempt` can be used to make it look like it has not committed any input)"] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char;"] # [doc = " # use combine::parser::byte;"] # [doc = " # use combine::parser::combinator::attempt;"] # [doc = " # use combine::parser::repeat::skip_until;"] # [doc = " # fn main() {"] # [doc = " let mut char_parser = skip_until(char::digit());"] # [doc = " assert_eq!(char_parser.parse(\"abc123\"), Ok(((), \"123\")));"] # [doc = ""] # [doc = " let mut byte_parser = skip_until(byte::bytes(&b\"TAG\"[..]));"] # [doc = " assert_eq!(byte_parser.parse(&b\"123TAG\"[..]), Ok(((), &b\"TAG\"[..])));"] # [doc = " assert!(byte_parser.parse(&b\"123TATAG\"[..]).is_err());"] # [doc = ""] # [doc = " // `attempt` must be used if the `end` should consume input before failing"] # [doc = " let mut byte_parser = skip_until(attempt(byte::bytes(&b\"TAG\"[..])));"] # [doc = " assert_eq!(byte_parser.parse(&b\"123TATAG\"[..]), Ok(((), &b\"TAG\"[..])));"] # [doc = " # }"] # [doc = " ```"] pub fn skip_until [Input , P] (end : P) (Input) -> () where [P : Parser < Input >,] { take_until ::< Sink , _ , _ > (end) . with (value (())) } }
};
}
