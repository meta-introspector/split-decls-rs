// Generated macro for roundevenf16 (function)
macro_rules! Depcrate_math_roundevenroundevenf16 {
() => {
// Module: crate::math::roundeven
// Provides: {"roundevenf16"}
// Dependencies: {}
# [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf16 (x : f16) -> f16 { roundeven_impl (x) }
};
}
