// Generated macro for fmodf (function)
macro_rules! Depcrate_math_fmodfmodf {
() => {
// Module: crate::math::fmod
// Provides: {"fmodf"}
// Dependencies: {}
# [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf (x : f32 , y : f32) -> f32 { super :: generic :: fmod (x , y) }
};
}
