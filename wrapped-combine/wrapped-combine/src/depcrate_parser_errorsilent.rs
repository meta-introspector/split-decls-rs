// Generated macro for silent (function)
macro_rules! Depcrate_parser_errorsilent {
() => {
// Module: crate::parser::error
// Provides: {"silent"}
// Dependencies: {}
# [doc = " Equivalent to [`p.silent()`]."] # [doc = ""] # [doc = " [`p.silent()`]: ../trait.Parser.html#method.silent"] pub fn silent < Input , P > (p : P) -> Silent < P > where P : Parser < Input > , Input : Stream , { Silent (p) }
};
}
