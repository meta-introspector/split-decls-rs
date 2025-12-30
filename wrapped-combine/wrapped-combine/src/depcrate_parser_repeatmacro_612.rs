// Generated macro for macro_612 (macro)
macro_rules! Depcrate_parser_repeatmacro_612 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_612"}
// Dependencies: {}
parser ! { pub struct SkipCountMinMax ; type PartialState = < With < CountMinMax < Sink , P >, Value < Input , () >> as Parser < Input >>:: PartialState ; # [doc = " Parses `parser` from `min` to `max` times (including `min` and `max`)"] # [doc = " skipping the output of `parser`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = skip_count_min_max(2, 2, token(b'a'));"] # [doc = ""] # [doc = " let result = parser.parse(&b\"aaab\"[..]);"] # [doc = " assert_eq!(result, Ok(((), &b\"ab\"[..])));"] # [doc = " let result = parser.parse(&b\"ab\"[..]);"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `min` > `max`."] pub fn skip_count_min_max [Input , P] (min : usize , max : usize , parser : P) (Input) -> () where [P : Parser < Input >,] { count_min_max ::< Sink , _ , _ > (* min , * max , parser . map (| _ | ())) . with (value (())) } }
};
}
