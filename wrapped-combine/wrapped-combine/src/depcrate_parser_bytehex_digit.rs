// Generated macro for hex_digit (function)
macro_rules! Depcrate_parser_bytehex_digit {
() => {
// Module: crate::parser::byte
// Provides: {"hex_digit"}
// Dependencies: {}
# [doc = " Parses an ASCII hexdecimal digit (accepts both uppercase and lowercase)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::hex_digit;"] # [doc = " assert_eq!(hex_digit().parse(&b\"F\"[..]), Ok((b'F', &b\"\"[..])));"] # [doc = " assert!(hex_digit().parse(&b\"H\"[..]).is_err());"] # [doc = " ```"] pub fn hex_digit < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (hex_digit , HexDigit , is_ascii_hexdigit ()) }
};
}
