// Generated macro for letter (function)
macro_rules! Depcrate_parser_charletter {
() => {
// Module: crate::parser::char
// Provides: {"letter"}
// Dependencies: {}
# [doc = " Parses an alphabet letter according to [`std::char::is_alphabetic`]."] # [doc = ""] # [doc = " [`std::char::is_alphabetic`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_alphabetic"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::letter;"] # [doc = " assert_eq!(letter().parse(\"a\"), Ok(('a', \"\")));"] # [doc = " assert_eq!(letter().parse(\"A\"), Ok(('A', \"\")));"] # [doc = " assert!(letter().parse(\"9\").is_err());"] # [doc = " ```"] pub fn letter < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_alphabetic ()) . expected ("letter") }
};
}
