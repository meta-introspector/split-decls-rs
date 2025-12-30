// Generated macro for upper (function)
macro_rules! Depcrate_parser_byteupper {
() => {
// Module: crate::parser::byte
// Provides: {"upper"}
// Dependencies: {}
# [doc = " Parses an uppercase ASCII letter (A–Z)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::upper;"] # [doc = " assert_eq!(upper().parse(&b\"A\"[..]), Ok((b'A', &b\"\"[..])));"] # [doc = " assert!(upper().parse(&b\"a\"[..]).is_err());"] # [doc = " ```"] pub fn upper < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (upper , Upper , is_ascii_uppercase) }
};
}
