// Generated macro for tab (function)
macro_rules! Depcrate_parser_bytetab {
() => {
// Module: crate::parser::byte
// Provides: {"tab"}
// Dependencies: {}
# [doc = " Parses a tab byte (`b'\\t'`)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::tab;"] # [doc = " assert_eq!(tab().parse(&b\"\\t\"[..]), Ok((b'\\t', &b\"\"[..])));"] # [doc = " assert!(tab().parse(&b\" \"[..]).is_err());"] # [doc = " ```"] pub fn tab < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { satisfy (| ch | ch == b'\t') . expected ("tab") }
};
}
