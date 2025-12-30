// Generated macro for crlf (function)
macro_rules! Depcrate_parser_charcrlf {
() => {
// Module: crate::parser::char
// Provides: {"crlf"}
// Dependencies: {}
# [doc = " Parses carriage return and newline (`\"\\r\\n\"`), returning the newline character."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::crlf;"] # [doc = " assert_eq!(crlf().parse(\"\\r\\n\"), Ok(('\\n', \"\")));"] # [doc = " assert!(crlf().parse(\"\\r\").is_err());"] # [doc = " assert!(crlf().parse(\"\\n\").is_err());"] # [doc = " ```"] pub fn crlf < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { no_partial (satisfy (| ch : char | ch == '\r') . with (newline ())) . expected ("crlf newline") }
};
}
