// Generated macro for char (function)
macro_rules! Depcrate_parser_charchar {
() => {
// Module: crate::parser::char
// Provides: {"char"}
// Dependencies: {}
# [doc = " Parses a character and succeeds if the character is equal to `c`."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::char;"] # [doc = " assert_eq!(char('!').parse(\"!\"), Ok(('!', \"\")));"] # [doc = " assert!(char('A').parse(\"!\").is_err());"] # [doc = " ```"] pub fn char < Input > (c : char) -> Token < Input > where Input : Stream < Token = char > , { token (c) }
};
}
