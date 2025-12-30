// Generated macro for newline (function)
macro_rules! Depcrate_parser_charnewline {
() => {
// Module: crate::parser::char
// Provides: {"newline"}
// Dependencies: {}
# [doc = " Parses a newline character (`'\\n'`)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::newline;"] # [doc = " assert_eq!(newline().parse(\"\\n\"), Ok(('\\n', \"\")));"] # [doc = " assert!(newline().parse(\"\\r\").is_err());"] # [doc = " ```"] pub fn newline < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch == '\n') . expected ("lf newline") }
};
}
