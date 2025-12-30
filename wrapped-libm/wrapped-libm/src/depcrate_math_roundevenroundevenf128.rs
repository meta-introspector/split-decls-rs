// Generated macro for roundevenf128 (function)
macro_rules! Depcrate_math_roundevenroundevenf128 {
() => {
// Module: crate::math::roundeven
// Provides: {"roundevenf128"}
// Dependencies: {}
# [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf128 (x : f128) -> f128 { roundeven_impl (x) }
};
}
