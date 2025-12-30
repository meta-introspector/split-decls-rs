// Generated macro for ldexpf16 (function)
macro_rules! Depcrate_math_ldexpldexpf16 {
() => {
// Module: crate::math::ldexp
// Provides: {"ldexpf16"}
// Dependencies: {}
# [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf16 (x : f16 , n : i32) -> f16 { super :: scalbnf16 (x , n) }
};
}
