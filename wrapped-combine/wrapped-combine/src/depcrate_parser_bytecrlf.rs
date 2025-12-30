// Generated macro for crlf (function)
macro_rules! Depcrate_parser_bytecrlf {
() => {
// Module: crate::parser::byte
// Provides: {"crlf"}
// Dependencies: {}
# [doc = " Parses carriage return and newline (`&b\"\\r\\n\"`), returning the newline byte."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::crlf;"] # [doc = " assert_eq!(crlf().parse(&b\"\\r\\n\"[..]), Ok((b'\\n', &b\"\"[..])));"] # [doc = " assert!(crlf().parse(&b\"\\r\"[..]).is_err());"] # [doc = " assert!(crlf().parse(&b\"\\n\"[..]).is_err());"] # [doc = " ```"] pub fn crlf < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { no_partial (satisfy (| ch : u8 | ch == b'\r') . with (newline ())) . expected ("crlf newline") }
};
}
