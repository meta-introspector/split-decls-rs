// Generated macro for map (function)
macro_rules! Depcrate_parser_combinatormap {
() => {
// Module: crate::parser::combinator
// Provides: {"map"}
// Dependencies: {}
# [doc = " Equivalent to [`p.map(f)`]."] # [doc = ""] # [doc = " [`p.map(f)`]: ../trait.Parser.html#method.map"] pub fn map < Input , P , F , B > (p : P , f : F) -> Map < P , F > where Input : Stream , P : Parser < Input > , F : FnMut (P :: Output) -> B , { Map (p , f) }
};
}
