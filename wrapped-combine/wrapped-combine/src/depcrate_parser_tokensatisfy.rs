// Generated macro for satisfy (function)
macro_rules! Depcrate_parser_tokensatisfy {
() => {
// Module: crate::parser::token
// Provides: {"satisfy"}
// Dependencies: {}
# [doc = " Parses a token and succeeds depending on the result of `predicate`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let mut parser = satisfy(|c| c == '!' || c == '?');"] # [doc = " assert_eq!(parser.parse(\"!\").map(|x| x.0), Ok('!'));"] # [doc = " assert_eq!(parser.parse(\"?\").map(|x| x.0), Ok('?'));"] # [doc = " # }"] # [doc = " ```"] pub fn satisfy < Input , P > (predicate : P) -> Satisfy < Input , P > where Input : Stream , P : FnMut (Input :: Token) -> bool , { Satisfy { predicate , _marker : PhantomData , } }
};
}
