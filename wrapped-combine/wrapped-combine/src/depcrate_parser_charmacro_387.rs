// Generated macro for macro_387 (macro)
macro_rules! Depcrate_parser_charmacro_387 {
() => {
// Module: crate::parser::char
// Provides: {"macro_387"}
// Dependencies: {}
parser ! { # [derive (Copy , Clone)] pub struct Digit ; # [doc = " Parses a base-10 digit."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::digit;"] # [doc = " assert_eq!(digit().parse(\"9\"), Ok(('9', \"\")));"] # [doc = " assert!(digit().parse(\"A\").is_err());"] # [doc = " ```"] pub fn digit [Input] () (Input) -> char where [Input : Stream < Token = char >,] { satisfy (| c : char | c . is_digit (10)) . expected ("digit") } }
};
}
