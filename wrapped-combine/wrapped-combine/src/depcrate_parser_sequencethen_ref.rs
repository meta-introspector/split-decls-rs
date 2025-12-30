// Generated macro for then_ref (function)
macro_rules! Depcrate_parser_sequencethen_ref {
() => {
// Module: crate::parser::sequence
// Provides: {"then_ref"}
// Dependencies: {}
# [doc = " Equivalent to [`p.then_ref(f)`]."] # [doc = ""] # [doc = " [`p.then_ref(f)`]: ../trait.Parser.html#method.then"] pub fn then_ref < Input , P , F , N > (p : P , f : F) -> ThenRef < P , F > where Input : Stream , F : FnMut (& P :: Output) -> N , P : Parser < Input > , N : Parser < Input > , { ThenRef (p , f) }
};
}
