// Generated macro for token (function)
macro_rules! Depcrate_parser_tokentoken {
() => {
// Module: crate::parser::token
// Provides: {"token"}
// Dependencies: {}
# [doc = " Parses a character and succeeds if the character is equal to `c`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # fn main() {"] # [doc = " let result = token('!')"] # [doc = "     .parse(\"!\")"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok('!'));"] # [doc = " # }"] # [doc = " ```"] pub fn token < Input > (c : Input :: Token) -> Token < Input > where Input : Stream , Input :: Token : PartialEq , { Token { c , _marker : PhantomData , } }
};
}
