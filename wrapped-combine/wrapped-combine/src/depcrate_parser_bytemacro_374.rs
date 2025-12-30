// Generated macro for macro_374 (macro)
macro_rules! Depcrate_parser_bytemacro_374 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_374"}
// Dependencies: {}
parser ! { # [doc = " Parses the bytes `s`."] # [doc = ""] # [doc = " If you have a stream implementing [`RangeStream`] such as `&[u8]` you can also use the"] # [doc = " [`range`] parser which may be more efficient."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::byte::bytes;"] # [doc = " # fn main() {"] # [doc = " let result = bytes(&b\"rust\"[..])"] # [doc = "     .parse(&b\"rust\"[..])"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(&b\"rust\"[..]));"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [`RangeStream`]: super::super::stream::RangeStream"] # [doc = " [`range`]: super::range::range"] pub fn bytes ['a , 'b , Input] (s : &'static [u8]) (Input) -> &'a [u8] where [Input : Stream < Token = u8 , Range = &'b [u8] >,] { bytes_cmp (s , | l : u8 , r : u8 | l == r) } }
};
}
