// Generated macro for flat_map (function)
macro_rules! Depcrate_parser_combinatorflat_map {
() => {
// Module: crate::parser::combinator
// Provides: {"flat_map"}
// Dependencies: {}
# [doc = " Equivalent to [`p.flat_map(f)`]."] # [doc = ""] # [doc = " [`p.flat_map(f)`]: ../trait.Parser.html#method.flat_map"] pub fn flat_map < Input , P , F , B > (p : P , f : F) -> FlatMap < P , F > where Input : Stream , P : Parser < Input > , F : FnMut (P :: Output) -> Result < B , < Input as StreamOnce > :: Error > , { FlatMap (p , f) }
};
}
