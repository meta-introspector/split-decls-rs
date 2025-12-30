// Generated macro for hex_digit (function)
macro_rules! Depcrate_parser_charhex_digit {
() => {
// Module: crate::parser::char
// Provides: {"hex_digit"}
// Dependencies: {}
# [doc = " Parses a hexdecimal digit with uppercase and lowercase."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::hex_digit;"] # [doc = " assert_eq!(hex_digit().parse(\"F\"), Ok(('F', \"\")));"] # [doc = " assert!(hex_digit().parse(\"H\").is_err());"] # [doc = " ```"] pub fn hex_digit < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_digit (0x10)) . expected ("hexadecimal digit") }
};
}
