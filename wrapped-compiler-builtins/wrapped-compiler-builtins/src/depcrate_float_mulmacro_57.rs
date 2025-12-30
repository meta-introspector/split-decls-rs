// Generated macro for macro_57 (macro)
macro_rules! Depcrate_float_mulmacro_57 {
() => {
// Module: crate::float::mul
// Provides: {"macro_57"}
// Dependencies: {}
intrinsics ! { # [cfg (f16_enabled)] pub extern "C" fn __mulhf3 (a : f16 , b : f16) -> f16 { mul (a , b) } # [aapcs_on_arm] # [arm_aeabi_alias = __aeabi_fmul] pub extern "C" fn __mulsf3 (a : f32 , b : f32) -> f32 { mul (a , b) } # [aapcs_on_arm] # [arm_aeabi_alias = __aeabi_dmul] pub extern "C" fn __muldf3 (a : f64 , b : f64) -> f64 { mul (a , b) } # [ppc_alias = __mulkf3] # [cfg (f128_enabled)] pub extern "C" fn __multf3 (a : f128 , b : f128) -> f128 { mul (a , b) } }
};
}
