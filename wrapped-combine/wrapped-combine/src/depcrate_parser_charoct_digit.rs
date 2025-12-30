// Generated macro for oct_digit (function)
macro_rules! Depcrate_parser_charoct_digit {
() => {
// Module: crate::parser::char
// Provides: {"oct_digit"}
// Dependencies: {}
# [doc = " Parses an octal digit."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::oct_digit;"] # [doc = " assert_eq!(oct_digit().parse(\"7\"), Ok(('7', \"\")));"] # [doc = " assert!(oct_digit().parse(\"8\").is_err());"] # [doc = " ```"] pub fn oct_digit < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_digit (8)) . expected ("octal digit") }
};
}
