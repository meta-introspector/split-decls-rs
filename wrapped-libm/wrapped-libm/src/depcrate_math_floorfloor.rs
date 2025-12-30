// Generated macro for floor (function)
macro_rules! Depcrate_math_floorfloor {
() => {
// Module: crate::math::floor
// Provides: {"floor"}
// Dependencies: {}
# [doc = " Floor (f64)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floor (x : f64) -> f64 { select_implementation ! { name : floor , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , use_arch_required : all (target_arch = "x86" , not (target_feature = "sse2")) , args : x , } return super :: generic :: floor (x) ; }
};
}
