// Generated macro for macro_574 (macro)
macro_rules! Depcrate_parser_rangemacro_574 {
() => {
// Module: crate::parser::range
// Provides: {"macro_574"}
// Dependencies: {}
# [cfg (not (feature = "std"))] parser ! { # [doc = " Takes a parser which parses a `length` then extracts a range of that length and returns it."] # [doc = " Commonly used in binary formats"] # [doc = ""] # [doc = " ```"] # [doc = " # use combine::parser::{byte::num::be_u16, range::length_prefix};"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut input = Vec::new();"] # [doc = " input.extend_from_slice(&3u16.to_be_bytes());"] # [doc = " input.extend_from_slice(b\"1234\");"] # [doc = ""] # [doc = " let mut parser = length_prefix(be_u16());"] # [doc = " let result = parser.parse(&input[..]);"] # [doc = " assert_eq!(result, Ok((&b\"123\"[..], &b\"4\"[..])));"] # [doc = " # }"] # [doc = " ```"] pub fn length_prefix [Input , P] (len : P) (Input) -> Input :: Range where [Input : RangeStream , P : Parser < Input >, usize : TryFrom < P :: Output >, < usize as TryFrom < P :: Output >>:: Error : fmt :: Display + Send + Sync + 'static ,] { len . and_then (| u | { usize :: try_from (u) . map_err (StreamErrorFor ::< Input >:: message_format) }) . then_partial (|& mut len | take (len)) } }
};
}
