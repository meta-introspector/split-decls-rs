// Generated macro for macro_375 (macro)
macro_rules! Depcrate_parser_bytemacro_375 {
() => {
// Module: crate::parser::byte
// Provides: {"macro_375"}
// Dependencies: {}
parser ! { # [doc = " Parses the bytes `s` using `cmp` to compare each token."] # [doc = ""] # [doc = " If you have a stream implementing [`RangeStream`] such as `&[u8]` you can also use the"] # [doc = " [`range`] parser which may be more efficient."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::byte::bytes_cmp;"] # [doc = " # use combine::stream::easy::Info;"] # [doc = " # fn main() {"] # [doc = " let result = bytes_cmp(&b\"abc\"[..], |l, r| l.eq_ignore_ascii_case(&r))"] # [doc = "     .parse(&b\"AbC\"[..]);"] # [doc = " assert_eq!(result, Ok((&b\"abc\"[..], &b\"\"[..])));"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [`RangeStream`]: super::super::stream::RangeStream"] # [doc = " [`range`]: super::range::range"] pub fn bytes_cmp ['a , 'b , C , Input] (s : &'static [u8] , cmp : C) (Input) -> &'a [u8] where [C : FnMut (u8 , u8) -> bool , Input : Stream < Token = u8 , Range = &'b [u8] >,] { let s = * s ; tokens_cmp (s . iter () . cloned () , cmp) . map (move | _ | s) . expected (error :: Range (s)) } }
};
}
