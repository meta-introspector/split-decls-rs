// Generated macro for digit (function)
macro_rules! Depcrate_parser_bytedigit {
() => {
// Module: crate::parser::byte
// Provides: {"digit"}
// Dependencies: {}
# [doc = " Parses a base-10 digit (0–9)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::digit;"] # [doc = " assert_eq!(digit().parse(&b\"9\"[..]), Ok((b'9', &b\"\"[..])));"] # [doc = " assert!(digit().parse(&b\"A\"[..]).is_err());"] # [doc = " ```"] pub fn digit < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (digit , Digit , is_ascii_digit ()) }
};
}
