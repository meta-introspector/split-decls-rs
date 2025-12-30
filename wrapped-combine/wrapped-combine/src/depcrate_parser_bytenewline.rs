// Generated macro for newline (function)
macro_rules! Depcrate_parser_bytenewline {
() => {
// Module: crate::parser::byte
// Provides: {"newline"}
// Dependencies: {}
# [doc = " Parses a newline byte (`b'\\n'`)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::newline;"] # [doc = " assert_eq!(newline().parse(&b\"\\n\"[..]), Ok((b'\\n', &b\"\"[..])));"] # [doc = " assert!(newline().parse(&b\"\\r\"[..]).is_err());"] # [doc = " ```"] pub fn newline < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { satisfy (| ch : u8 | ch == b'\n') . expected ("lf newline") }
};
}
