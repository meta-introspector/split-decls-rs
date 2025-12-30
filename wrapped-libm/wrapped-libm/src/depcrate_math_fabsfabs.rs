// Generated macro for fabs (function)
macro_rules! Depcrate_math_fabsfabs {
() => {
// Module: crate::math::fabs
// Provides: {"fabs"}
// Dependencies: {}
# [doc = " Absolute value (magnitude) (f64)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabs (x : f64) -> f64 { select_implementation ! { name : fabs , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: fabs (x) }
};
}
