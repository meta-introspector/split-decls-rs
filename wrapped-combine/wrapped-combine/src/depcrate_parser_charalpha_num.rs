// Generated macro for alpha_num (function)
macro_rules! Depcrate_parser_charalpha_num {
() => {
// Module: crate::parser::char
// Provides: {"alpha_num"}
// Dependencies: {}
# [doc = " Parses either an alphabet letter or digit according to [`std::char::is_alphanumeric`]."] # [doc = ""] # [doc = " [`std::char::is_alphanumeric`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_alphanumeric"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::alpha_num;"] # [doc = " assert_eq!(alpha_num().parse(\"A\"), Ok(('A', \"\")));"] # [doc = " assert_eq!(alpha_num().parse(\"1\"), Ok(('1', \"\")));"] # [doc = " assert!(alpha_num().parse(\"!\").is_err());"] # [doc = " ```"] pub fn alpha_num < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { satisfy (| ch : char | ch . is_alphanumeric ()) . expected ("letter or digit") }
};
}
