// Generated macro for roundf16 (function)
macro_rules! Depcrate_math_roundroundf16 {
() => {
// Module: crate::math::round
// Provides: {"roundf16"}
// Dependencies: {}
# [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf16 (x : f16) -> f16 { super :: generic :: round (x) }
};
}
