// Generated macro for lower (function)
macro_rules! Depcrate_parser_charlower {
() => {
// Module: crate::parser::char
// Provides: {"lower"}
// Dependencies: {}
# [doc = " Parses an lowercase letter according to [`std::char::is_lowercase`]."] # [doc = ""] # [doc = " [`std::char::is_lowercase`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_lowercase"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::lower;"] # [doc = " assert_eq!(lower().parse(\"a\"), Ok(('a', \"\")));"] # [doc = " assert!(lower().parse(\"A\").is_err());"] # [doc = " ```"] pub fn lower < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_lowercase ()) . expected ("lowercase letter") }
};
}
