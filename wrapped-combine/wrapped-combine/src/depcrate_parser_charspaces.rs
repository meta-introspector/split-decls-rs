// Generated macro for spaces (function)
macro_rules! Depcrate_parser_charspaces {
() => {
// Module: crate::parser::char
// Provides: {"spaces"}
// Dependencies: {}
# [doc = " Skips over zero or more spaces according to [`std::char::is_whitespace`]."] # [doc = ""] # [doc = " This includes space characters, tabs and newlines."] # [doc = ""] # [doc = " [`std::char::is_whitespace`]: https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::char::spaces;"] # [doc = " assert_eq!(spaces().parse(\"\"), Ok(((), \"\")));"] # [doc = " assert_eq!(spaces().parse(\"   \"), Ok(((), \"\")));"] # [doc = " ```"] pub fn spaces < Input > () -> impl Parser < Input , Output = () > where Input : Stream < Token = char > , { skip_many (space ()) . expected ("whitespaces") }
};
}
