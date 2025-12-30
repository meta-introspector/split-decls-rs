// Generated macro for spaces (function)
macro_rules! Depcrate_parser_bytespaces {
() => {
// Module: crate::parser::byte
// Provides: {"spaces"}
// Dependencies: {}
# [doc = " Skips over [`space`] zero or more times"] # [doc = ""] # [doc = " [`space`]: fn.space.html"] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::spaces;"] # [doc = " assert_eq!(spaces().parse(&b\"\"[..]), Ok(((), &b\"\"[..])));"] # [doc = " assert_eq!(spaces().parse(&b\"   \"[..]), Ok(((), &b\"\"[..])));"] # [doc = " ```"] pub fn spaces < Input > () -> impl Parser < Input , Output = () > where Input : Stream < Token = u8 > , { skip_many (space ()) . expected ("whitespaces") }
};
}
