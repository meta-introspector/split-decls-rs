// Generated macro for none_of (function)
macro_rules! Depcrate_parser_tokennone_of {
() => {
// Module: crate::parser::token
// Provides: {"none_of"}
// Dependencies: {}
# [doc = " Extract one token and succeeds if it is not part of `tokens`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::stream::easy;"] # [doc = " # use combine::stream::position;"] # [doc = " # fn main() {"] # [doc = " let mut parser = many1(none_of(b\"abc\".iter().cloned()));"] # [doc = " let result = parser.easy_parse(position::Stream::new(&b\"xyb\"[..]))"] # [doc = "     .map(|(output, input)| (output, input.input));"] # [doc = " assert_eq!(result, Ok((b\"xy\"[..].to_owned(), &b\"b\"[..])));"] # [doc = ""] # [doc = " let result = parser.easy_parse(position::Stream::new(&b\"ab\"[..]));"] # [doc = " assert_eq!(result, Err(easy::Errors {"] # [doc = "     position: 0,"] # [doc = "     errors: vec!["] # [doc = "         easy::Error::Unexpected(easy::Info::Token(b'a')),"] # [doc = "     ]"] # [doc = " }));"] # [doc = " # }"] # [doc = " ```"] pub fn none_of < T , Input > (tokens : T) -> NoneOf < T , Input > where T : Clone + IntoIterator , Input : Stream , Input :: Token : PartialEq < T :: Item > , { NoneOf { tokens , _marker : PhantomData , } }
};
}
