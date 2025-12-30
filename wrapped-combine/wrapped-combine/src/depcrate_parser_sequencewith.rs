// Generated macro for with (function)
macro_rules! Depcrate_parser_sequencewith {
() => {
// Module: crate::parser::sequence
// Provides: {"with"}
// Dependencies: {}
# [doc = " Equivalent to [`p1.with(p2)`]."] # [doc = ""] # [doc = " [`p1.with(p2)`]: ../trait.Parser.html#method.with"] pub fn with < Input , P1 , P2 > (p1 : P1 , p2 : P2) -> With < P1 , P2 > where Input : Stream , P1 : Parser < Input > , P2 : Parser < Input > , { With ((ignore (p1) , p2)) }
};
}
