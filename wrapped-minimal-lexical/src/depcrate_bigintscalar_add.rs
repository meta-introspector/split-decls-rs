// Generated macro for scalar_add (function)
macro_rules! Depcrate_bigintscalar_add {
() => {
// Module: crate::bigint
// Provides: {"scalar_add"}
// Dependencies: {}
# [doc = " Add two small integers and return the resulting value and if overflow happens."] # [inline (always)] pub fn scalar_add (x : Limb , y : Limb) -> (Limb , bool) { x . overflowing_add (y) }
};
}
