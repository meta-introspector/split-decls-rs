// Generated macro for tab (function)
macro_rules! Depcrate_parser_chartab {
() => {
// Module: crate::parser::char
// Provides: {"tab"}
// Dependencies: {}
# [doc = " Parses a tab character (`'\\t'`)."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::tab;"] # [doc = " assert_eq!(tab().parse(\"\\t\"), Ok(('\\t', \"\")));"] # [doc = " assert!(tab().parse(\" \").is_err());"] # [doc = " ```"] pub fn tab < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch == '\t') . expected ("tab") }
};
}
