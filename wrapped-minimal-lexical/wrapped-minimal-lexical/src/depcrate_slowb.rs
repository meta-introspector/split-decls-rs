// Generated macro for b (function)
macro_rules! Depcrate_slowb {
() => {
// Module: crate::slow
// Provides: {"b"}
// Dependencies: {}
# [doc = " Calculate `b` from a a representation of `b` as a float."] # [inline] pub fn b < F : Float > (float : F) -> ExtendedFloat { ExtendedFloat { mant : float . mantissa () , exp : float . exponent () , } }
};
}
