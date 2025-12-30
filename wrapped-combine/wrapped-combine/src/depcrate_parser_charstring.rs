// Generated macro for string (function)
macro_rules! Depcrate_parser_charstring {
() => {
// Module: crate::parser::char
// Provides: {"string"}
// Dependencies: {}
# [doc = " Parses the string `s`."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::string;"] # [doc = " # fn main() {"] # [doc = " let result = string(\"rust\")"] # [doc = "     .parse(\"rust\")"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(\"rust\"));"] # [doc = " # }"] # [doc = " ```"] pub fn string < 'a , Input > (s : & 'static str) -> impl Parser < Input , Output = & 'a str > where Input : Stream < Token = char > , { string_cmp (s , | l , r | l == r) }
};
}
