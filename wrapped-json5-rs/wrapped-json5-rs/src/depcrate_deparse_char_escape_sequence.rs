// Generated macro for parse_char_escape_sequence (function)
macro_rules! Depcrate_deparse_char_escape_sequence {
() => {
// Module: crate::de
// Provides: {"parse_char_escape_sequence"}
// Dependencies: {}
fn parse_char_escape_sequence < 'a > (pair : & 'a Pair < '_ , Rule >) -> & 'a str { match pair . as_str () { "b" => "\u{0008}" , "f" => "\u{000C}" , "n" => "\n" , "r" => "\r" , "t" => "\t" , "v" => "\u{000B}" , c => c , } }
};
}
