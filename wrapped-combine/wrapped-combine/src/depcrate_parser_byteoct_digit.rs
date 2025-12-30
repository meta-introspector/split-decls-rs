// Generated macro for oct_digit (function)
macro_rules! Depcrate_parser_byteoct_digit {
() => {
// Module: crate::parser::byte
// Provides: {"oct_digit"}
// Dependencies: {}
# [doc = " Parses an octal digit."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::oct_digit;"] # [doc = " assert_eq!(oct_digit().parse(&b\"7\"[..]), Ok((b'7', &b\"\"[..])));"] # [doc = " assert!(oct_digit().parse(&b\"8\"[..]).is_err());"] # [doc = " ```"] pub fn oct_digit < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { satisfy (| ch | (b'0' ..= b'7') . contains (& ch)) . expected ("octal digit") }
};
}
