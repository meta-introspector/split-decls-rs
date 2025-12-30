// Generated macro for ParseHex (trait)
macro_rules! Depcrate_parserParseHex {
() => {
// Module: crate::parser
// Provides: {"ParseHex"}
// Dependencies: {}
# [doc = "\nParse a value from a hex string.\n"] pub trait ParseHex { # [doc = " Parse the value from hex."] fn parse_hex (input : & str) -> Result < Self , ParseError > where Self : Sized ; }
};
}
