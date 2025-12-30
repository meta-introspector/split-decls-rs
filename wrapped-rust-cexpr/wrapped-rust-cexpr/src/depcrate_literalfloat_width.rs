// Generated macro for float_width (function)
macro_rules! Depcrate_literalfloat_width {
() => {
// Module: crate::literal
// Provides: {"float_width"}
// Dependencies: {}
fn float_width (i : & [u8]) -> nom :: IResult < & [u8] , u8 > { nom :: combinator :: complete (byte ! (b'f' | b'l' | b'F' | b'L')) (i) }
};
}
