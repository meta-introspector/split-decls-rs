// Generated macro for lower (function)
macro_rules! Depcrate_parser_bytelower {
() => {
// Module: crate::parser::byte
// Provides: {"lower"}
// Dependencies: {}
# [doc = " Parses an lowercase ASCII letter (a–z)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::lower;"] # [doc = " assert_eq!(lower().parse(&b\"a\"[..]), Ok((b'a', &b\"\"[..])));"] # [doc = " assert!(lower().parse(&b\"A\"[..]).is_err());"] # [doc = " ```"] pub fn lower < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (lower , Lower , is_ascii_lowercase) }
};
}
