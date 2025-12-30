// Generated macro for upper (function)
macro_rules! Depcrate_parser_charupper {
() => {
// Module: crate::parser::char
// Provides: {"upper"}
// Dependencies: {}
# [doc = " Parses an uppercase letter according to [`std::char::is_uppercase`]."] # [doc = ""] # [doc = " [`std::char::is_uppercase`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_uppercase"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::upper;"] # [doc = " assert_eq!(upper().parse(\"A\"), Ok(('A', \"\")));"] # [doc = " assert!(upper().parse(\"a\").is_err());"] # [doc = " ```"] pub fn upper < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_uppercase ()) . expected ("uppercase letter") }
};
}
