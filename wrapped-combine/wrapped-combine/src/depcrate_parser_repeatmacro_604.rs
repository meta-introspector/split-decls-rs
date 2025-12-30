// Generated macro for macro_604 (macro)
macro_rules! Depcrate_parser_repeatmacro_604 {
() => {
// Module: crate::parser::repeat
// Provides: {"macro_604"}
// Dependencies: {}
parser ! { pub struct Count ; # [doc = " Parses `parser` from zero up to `count` times."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::error::Info;"] # [doc = " # use combine::stream::easy::Error;"] # [doc = " # fn main() {"] # [doc = " let mut parser = count(2, token(b'a'));"] # [doc = ""] # [doc = " let result = parser.parse(&b\"aaab\"[..]);"] # [doc = " assert_eq!(result, Ok((b\"aa\"[..].to_owned(), &b\"ab\"[..])));"] # [doc = " # }"] # [doc = " ```"] pub fn count [F , Input , P] (count : usize , parser : P) (Input) -> F where [Input : Stream , P : Parser < Input >, F : Extend < P :: Output > + Default ,] { count_min_max (0 , * count , parser) } }
};
}
