// Generated macro for macro_550 (macro)
macro_rules! Depcrate_parser_rangemacro_550 {
() => {
// Module: crate::parser::range
// Provides: {"macro_550"}
// Dependencies: {}
parser ! { # [derive (Clone)] pub struct Recognize ; type PartialState = < RecognizeWithValue < P > as Parser < Input >>:: PartialState ; # [doc = " Zero-copy parser which returns committed input range."] # [doc = ""] # [doc = " [`combinator::recognize`][] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`combinator::recognize`]: ../../parser/combinator/fn.recognize.html"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::recognize;"] # [doc = " # use combine::parser::char::letter;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = recognize(skip_many1(letter()));"] # [doc = " assert_eq!(parser.parse(\"hello world\"), Ok((\"hello\", \" world\")));"] # [doc = " assert!(parser.parse(\"!\").is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn recognize [Input , P] (parser : P) (Input) -> < Input as StreamOnce >:: Range where [P : Parser < Input >, Input : RangeStream , < Input as StreamOnce >:: Range : crate :: stream :: Range ,] { recognize_with_value (parser) . map (| (range , _) | range) } }
};
}
