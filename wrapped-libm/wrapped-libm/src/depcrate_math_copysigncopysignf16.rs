// Generated macro for copysignf16 (function)
macro_rules! Depcrate_math_copysigncopysignf16 {
() => {
// Module: crate::math::copysign
// Provides: {"copysignf16"}
// Dependencies: {}
# [doc = " Sign of Y, magnitude of X (f16)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf16 (x : f16 , y : f16) -> f16 { super :: generic :: copysign (x , y) }
};
}
