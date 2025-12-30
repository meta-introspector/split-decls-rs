// Generated macro for take_while (function)
macro_rules! Depcrate_parser_rangetake_while {
() => {
// Module: crate::parser::range
// Provides: {"take_while"}
// Dependencies: {}
# [doc = " Zero-copy parser which reads a range of 0 or more tokens which satisfy `f`."] # [doc = ""] # [doc = " [`many`][] is a non-`RangeStream` alternative."] # [doc = ""] # [doc = " [`many`]: ../../parser/repeat/fn.many.html"] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::parser::range::take_while;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = take_while(|c: char| c.is_digit(10));"] # [doc = " let result = parser.parse(\"123abc\");"] # [doc = " assert_eq!(result, Ok((\"123\", \"abc\")));"] # [doc = " let result = parser.parse(\"abc\");"] # [doc = " assert_eq!(result, Ok((\"\", \"abc\")));"] # [doc = " # }"] # [doc = " ```"] pub fn take_while < Input , F > (f : F) -> TakeWhile < Input , F > where Input : RangeStream , Input :: Range : crate :: stream :: Range , F : FnMut (Input :: Token) -> bool , { TakeWhile (f , PhantomData) }
};
}
