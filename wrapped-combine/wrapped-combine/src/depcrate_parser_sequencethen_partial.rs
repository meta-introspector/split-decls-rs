// Generated macro for then_partial (function)
macro_rules! Depcrate_parser_sequencethen_partial {
() => {
// Module: crate::parser::sequence
// Provides: {"then_partial"}
// Dependencies: {}
# [doc = " Equivalent to [`p.then_partial(f)`]."] # [doc = ""] # [doc = " [`p.then_partial(f)`]: ../trait.Parser.html#method.then_partial"] pub fn then_partial < Input , P , F , N > (p : P , f : F) -> ThenPartial < P , F > where Input : Stream , F : FnMut (& mut P :: Output) -> N , P : Parser < Input > , N : Parser < Input > , { ThenPartial (p , f) }
};
}
