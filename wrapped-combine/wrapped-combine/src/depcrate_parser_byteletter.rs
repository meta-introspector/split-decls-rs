// Generated macro for letter (function)
macro_rules! Depcrate_parser_byteletter {
() => {
// Module: crate::parser::byte
// Provides: {"letter"}
// Dependencies: {}
# [doc = " Parses an ASCII alphabet letter (a–z, A–Z)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::letter;"] # [doc = " assert_eq!(letter().parse(&b\"a\"[..]), Ok((b'a', &b\"\"[..])));"] # [doc = " assert_eq!(letter().parse(&b\"A\"[..]), Ok((b'A', &b\"\"[..])));"] # [doc = " assert!(letter().parse(&b\"9\"[..]).is_err());"] # [doc = " ```"] pub fn letter < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (letter , Letter , is_ascii_alphabetic) }
};
}
