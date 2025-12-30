// Generated macro for copysignf (function)
macro_rules! Depcrate_math_copysigncopysignf {
() => {
// Module: crate::math::copysign
// Provides: {"copysignf"}
// Dependencies: {}
# [doc = " Sign of Y, magnitude of X (f32)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf (x : f32 , y : f32) -> f32 { super :: generic :: copysign (x , y) }
};
}
