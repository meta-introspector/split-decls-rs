// Generated macro for string_cmp (function)
macro_rules! Depcrate_parser_charstring_cmp {
() => {
// Module: crate::parser::char
// Provides: {"string_cmp"}
// Dependencies: {}
# [doc = " Parses the string `s`, using `cmp` to compare each character."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate combine;"] # [doc = " # use combine::*;"] # [doc = " # use combine::parser::char::string_cmp;"] # [doc = " # fn main() {"] # [doc = " let result = string_cmp(\"rust\", |l, r| l.eq_ignore_ascii_case(&r))"] # [doc = "     .parse(\"RusT\")"] # [doc = "     .map(|x| x.0);"] # [doc = " assert_eq!(result, Ok(\"rust\"));"] # [doc = " # }"] # [doc = " ```"] pub fn string_cmp < 'a , C , Input > (s : & 'static str , cmp : C) -> impl Parser < Input , Output = & 'a str > where C : FnMut (char , char) -> bool , Input : Stream < Token = char > , { tokens_cmp (s . chars () , cmp) . map (move | _ | s) . expected (s) }
};
}
