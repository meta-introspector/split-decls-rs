// Generated macro for alpha_num (function)
macro_rules! Depcrate_parser_bytealpha_num {
() => {
// Module: crate::parser::byte
// Provides: {"alpha_num"}
// Dependencies: {}
# [doc = " Parses either an ASCII alphabet letter or digit (a–z, A–Z, 0–9)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::alpha_num;"] # [doc = " assert_eq!(alpha_num().parse(&b\"A\"[..]), Ok((b'A', &b\"\"[..])));"] # [doc = " assert_eq!(alpha_num().parse(&b\"1\"[..]), Ok((b'1', &b\"\"[..])));"] # [doc = " assert!(alpha_num().parse(&b\"!\"[..]).is_err());"] # [doc = " ```"] pub fn alpha_num < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (alpha_num , AlphaNum , is_ascii_alphanumeric) }
};
}
