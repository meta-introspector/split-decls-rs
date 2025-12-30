// Generated macro for macro_605 (macro)
macro_rules! Depcrate_parser_repeatmacro_605 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_605"}
// Dependencies: {}
parser ! { pub struct SkipCount ; type PartialState = < With < Count < Sink , Input , P >, Value < Input , () >> as Parser < Input >>:: PartialState ; # [doc = " Parses `parser` from zero up to `count` times skipping the output of `parser`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::stream::easy::{Error, Info};"] # [doc = " # fn main() {"] # [doc = " let mut parser = skip_count(2, token(b'a'));"] # [doc = ""] # [doc = " let result = parser.parse(&b\"aaab\"[..]);"] # [doc = " assert_eq!(result, Ok(((), &b\"ab\"[..])));"] # [doc = " # }"] # [doc = " ```"] pub fn skip_count [Input , P] (count : usize , parser : P) (Input) -> () where [P : Parser < Input >] { self :: count ::< Sink , _ , _ > (* count , parser . map (| _ | ())) . with (value (())) } }
};
}
