// Generated macro for map_input (function)
macro_rules! Depcrate_parser_combinatormap_input {
() => {
// Module: crate::parser::combinator
// Provides: {"map_input"}
// Dependencies: {}
# [doc = " Equivalent to [`p.map_input(f)`]."] # [doc = ""] # [doc = " [`p.map_input(f)`]: ../trait.Parser.html#method.map_input"] pub fn map_input < Input , P , F , B > (p : P , f : F) -> MapInput < P , F > where Input : Stream , P : Parser < Input > , F : FnMut (P :: Output , & mut Input) -> B , { MapInput (p , f) }
};
}
