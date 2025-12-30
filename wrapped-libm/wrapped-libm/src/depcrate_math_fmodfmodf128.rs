// Generated macro for fmodf128 (function)
macro_rules! Depcrate_math_fmodfmodf128 {
() => {
// Module: crate::math::fmod
// Provides: {"fmodf128"}
// Dependencies: {}
# [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmod (x , y) }
};
}
