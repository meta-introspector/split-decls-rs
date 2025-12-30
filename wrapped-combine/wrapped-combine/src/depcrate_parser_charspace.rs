// Generated macro for space (function)
macro_rules! Depcrate_parser_charspace {
() => {
// Module: crate::parser::char
// Provides: {"space"}
// Dependencies: {}
# [doc = " Parse a single whitespace according to [`std::char::is_whitespace`]."] # [doc = ""] # [doc = " This includes space characters, tabs and newlines."] # [doc = ""] # [doc = " [`std::char::is_whitespace`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::space;"] # [doc = " assert_eq!(space().parse(\" \"), Ok((' ', \"\")));"] # [doc = " assert_eq!(space().parse(\"  \"), Ok((' ', \" \")));"] # [doc = " assert!(space().parse(\"!\").is_err());"] # [doc = " assert!(space().parse(\"\").is_err());"] # [doc = " ```"] pub fn space < Input > () -> impl Parser < Input , Output = char , PartialState = () > where Input : Stream < Token = char > , { let f : fn (char) -> bool = char :: is_whitespace ; satisfy (f) . expected ("whitespace") }
};
}
