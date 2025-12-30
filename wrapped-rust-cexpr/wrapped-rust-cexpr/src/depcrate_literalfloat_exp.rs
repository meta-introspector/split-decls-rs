// Generated macro for float_exp (function)
macro_rules! Depcrate_literalfloat_exp {
() => {
// Module: crate::literal
// Provides: {"float_exp"}
// Dependencies: {}
fn float_exp (i : & [u8]) -> nom :: IResult < & [u8] , (Option < u8 > , Vec < u8 >) > { preceded (byte ! (b'e' | b'E') , pair (opt (byte ! (b'-' | b'+')) , many1 (complete (decimal))) ,) (i) }
};
}
