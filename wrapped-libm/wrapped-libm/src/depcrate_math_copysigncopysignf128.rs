// Generated macro for copysignf128 (function)
macro_rules! Depcrate_math_copysigncopysignf128 {
() => {
// Module: crate::math::copysign
// Provides: {"copysignf128"}
// Dependencies: {}
# [doc = " Sign of Y, magnitude of X (f128)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf128 (x : f128 , y : f128) -> f128 { super :: generic :: copysign (x , y) }
};
}
