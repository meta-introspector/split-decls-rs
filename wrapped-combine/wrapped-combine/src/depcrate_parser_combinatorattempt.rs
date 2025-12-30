// Generated macro for attempt (function)
macro_rules! Depcrate_parser_combinatorattempt {
() => {
// Module: crate::parser::combinator
// Provides: {"attempt"}
// Dependencies: {}
# [doc = " `attempt(p)` behaves as `p` except it always acts as `p` peeked instead of committed on its"] # [doc = " parse."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::string;"] # [doc = " # fn main() {"] # [doc = " let mut p = attempt(string(\"let\"))"] # [doc = "     .or(string(\"lex\"));"] # [doc = " let result = p.parse(\"lex\").map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(\"lex\"));"] # [doc = " let result = p.parse(\"aet\").map(|x| x.0);"] # [doc = " assert!(result.is_err());"] # [doc = " # }"] # [doc = " ```"] pub fn attempt < Input , P > (p : P) -> Try < P > where Input : Stream , P : Parser < Input > , { Try (p) }
};
}
