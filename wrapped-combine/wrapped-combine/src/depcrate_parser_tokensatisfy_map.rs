// Generated macro for satisfy_map (function)
macro_rules! Depcrate_parser_tokensatisfy_map {
() => {
// Module: crate::parser::token
// Provides: {"satisfy_map"}
// Dependencies: {}
# [doc = " Parses a token and passes it to `predicate`. If `predicate` returns `Some` the parser succeeds"] # [doc = " and returns the value inside the `Option`. If `predicate` returns `None` the parser fails"] # [doc = " without consuming any input."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " #[derive(Debug, PartialEq)]"] # [doc = " enum YesNo {"] # [doc = "     Yes,"] # [doc = "     No,"] # [doc = " }"] # [doc = " let mut parser = satisfy_map(|c| {"] # [doc = "     match c {"] # [doc = "         'Y' => Some(YesNo::Yes),"] # [doc = "         'N' => Some(YesNo::No),"] # [doc = "         _ => None,"] # [doc = "     }"] # [doc = " });"] # [doc = " assert_eq!(parser.parse(\"Y\").map(|x| x.0), Ok(YesNo::Yes));"] # [doc = " assert!(parser.parse(\"A\").map(|x| x.0).is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn satisfy_map < Input , P , R > (predicate : P) -> SatisfyMap < Input , P > where Input : Stream , P : FnMut (Input :: Token) -> Option < R > , { SatisfyMap { predicate , _marker : PhantomData , } }
};
}
