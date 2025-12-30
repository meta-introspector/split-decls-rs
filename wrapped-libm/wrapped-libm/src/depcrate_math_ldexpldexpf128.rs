// Generated macro for ldexpf128 (function)
macro_rules! Depcrate_math_ldexpldexpf128 {
() => {
// Module: crate::math::ldexp
// Provides: {"ldexpf128"}
// Dependencies: {}
# [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf128 (x : f128 , n : i32) -> f128 { super :: scalbnf128 (x , n) }
};
}
