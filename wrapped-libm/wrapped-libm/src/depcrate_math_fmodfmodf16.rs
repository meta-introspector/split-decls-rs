// Generated macro for fmodf16 (function)
macro_rules! Depcrate_math_fmodfmodf16 {
() => {
// Module: crate::math::fmod
// Provides: {"fmodf16"}
// Dependencies: {}
# [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmod (x , y) }
};
}
