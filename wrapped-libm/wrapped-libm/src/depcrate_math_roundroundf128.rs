// Generated macro for roundf128 (function)
macro_rules! Depcrate_math_roundroundf128 {
() => {
// Module: crate::math::round
// Provides: {"roundf128"}
// Dependencies: {}
# [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf128 (x : f128) -> f128 { super :: generic :: round (x) }
};
}
