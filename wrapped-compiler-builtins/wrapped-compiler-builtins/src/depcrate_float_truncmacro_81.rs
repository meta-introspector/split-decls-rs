// Generated macro for macro_81 (macro)
macro_rules! Depcrate_float_truncmacro_81 {
() => {
// Module: crate::float::trunc
// Provides: {"macro_81"}
// Dependencies: {}
intrinsics ! { # [aapcs_on_arm] # [apple_f16_ret_abi] # [arm_aeabi_alias = __aeabi_f2h] # [cfg (f16_enabled)] pub extern "C" fn __truncsfhf2 (a : f32) -> f16 { trunc (a) } # [aapcs_on_arm] # [apple_f16_ret_abi] # [cfg (f16_enabled)] pub extern "C" fn __gnu_f2h_ieee (a : f32) -> f16 { trunc (a) } # [aapcs_on_arm] # [apple_f16_ret_abi] # [arm_aeabi_alias = __aeabi_d2h] # [cfg (f16_enabled)] pub extern "C" fn __truncdfhf2 (a : f64) -> f16 { trunc (a) } # [aapcs_on_arm] # [ppc_alias = __trunckfhf2] # [cfg (all (f16_enabled , f128_enabled))] pub extern "C" fn __trunctfhf2 (a : f128) -> f16 { trunc (a) } # [aapcs_on_arm] # [ppc_alias = __trunckfsf2] # [cfg (f128_enabled)] pub extern "C" fn __trunctfsf2 (a : f128) -> f32 { trunc (a) } # [aapcs_on_arm] # [ppc_alias = __trunckfdf2] # [cfg (f128_enabled)] pub extern "C" fn __trunctfdf2 (a : f128) -> f64 { trunc (a) } }
};
}
