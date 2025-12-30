// Generated macro for count_min_max (function)
macro_rules! Depcrate_parser_repeatcount_min_max {
() => {
// Module: crate::parser::repeat
// Provides: {"count_min_max"}
// Dependencies: {}
# [doc = " Parses `parser` from `min` to `max` times (including `min` and `max`)."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::stream::easy::{Error, Info};"] # [doc = " # fn main() {"] # [doc = " let mut parser = count_min_max(2, 2, token(b'a'));"] # [doc = ""] # [doc = " let result = parser.parse(&b\"aaab\"[..]);"] # [doc = " assert_eq!(result, Ok((b\"aa\"[..].to_owned(), &b\"ab\"[..])));"] # [doc = " let result = parser.parse(&b\"ab\"[..]);"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If `min` > `max`."] pub fn count_min_max < F , Input , P > (min : usize , max : usize , parser : P) -> CountMinMax < F , P > where Input : Stream , P : Parser < Input > , F : Extend < P :: Output > + Default , { assert ! (min <= max) ; CountMinMax { parser , min , max , _marker : PhantomData , } }
};
}
