// Generated macro for stag (function)
macro_rules! Depcrate_parse_utilstag {
() => {
// Module: crate::parse::util
// Provides: {"stag"}
// Dependencies: {}
# [doc = " Parsed a spaced tag."] pub fn stag (s : & str) -> impl Parser < '_ , & str > { spaced (tag (s)) }
};
}
