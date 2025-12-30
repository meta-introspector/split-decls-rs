// Generated macro for take_while1 (function)
macro_rules! Depcrate_parser_rangetake_while1 {
() => {
// Module: crate::parser::range
// Provides: {"take_while1"}
// Dependencies: {}
# [doc = " Zero-copy parser which reads a range of 1 or more tokens which satisfy `f`."] # [doc = ""] # [doc = " [`many1`][] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`many1`]: ../../parser/repeat/fn.many1.html"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::take_while1;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_while1(|c: char| c.is_digit(10));"] # [doc = " let result = parser.parse(\"123abc\");"] # [doc = " assert_eq!(result, Ok((\"123\", \"abc\")));"] # [doc = " let result = parser.parse(\"abc\");"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn take_while1 < Input , F > (f : F) -> TakeWhile1 < Input , F > where Input : RangeStream , Input :: Range : crate :: stream :: Range , F : FnMut (Input :: Token) -> bool , { TakeWhile1 (f , PhantomData) }
};
}
