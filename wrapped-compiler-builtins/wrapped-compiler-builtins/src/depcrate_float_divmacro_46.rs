// Generated macro for macro_46 (macro)
macro_rules! Depcrate_float_divmacro_46 {
() => {
// Module: crate::float::div
// Provides: {"macro_46"}
// Dependencies: {}
intrinsics ! { # [arm_aeabi_alias = __aeabi_fdiv] pub extern "C" fn __divsf3 (a : f32 , b : f32) -> f32 { div (a , b) } # [arm_aeabi_alias = __aeabi_ddiv] pub extern "C" fn __divdf3 (a : f64 , b : f64) -> f64 { div (a , b) } # [ppc_alias = __divkf3] # [cfg (f128_enabled)] pub extern "C" fn __divtf3 (a : f128 , b : f128) -> f128 { div (a , b) } # [cfg (target_arch = "arm")] pub extern "C" fn __divsf3vfp (a : f32 , b : f32) -> f32 { a / b } # [cfg (target_arch = "arm")] pub extern "C" fn __divdf3vfp (a : f64 , b : f64) -> f64 { a / b } }
};
}
