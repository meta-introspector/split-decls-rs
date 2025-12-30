// Generated macro for macro_52 (macro)
macro_rules! Depcrate_float_extendmacro_52 {
() => {
// Module: crate::float::extend
// Provides: {"macro_52"}
// Dependencies: {}
intrinsics ! { # [aapcs_on_arm] # [apple_f16_arg_abi] # [arm_aeabi_alias = __aeabi_h2f] # [cfg (f16_enabled)] pub extern "C" fn __extendhfsf2 (a : f16) -> f32 { extend (a) } # [aapcs_on_arm] # [apple_f16_arg_abi] # [cfg (f16_enabled)] pub extern "C" fn __gnu_h2f_ieee (a : f16) -> f32 { extend (a) } # [aapcs_on_arm] # [apple_f16_arg_abi] # [cfg (f16_enabled)] pub extern "C" fn __extendhfdf2 (a : f16) -> f64 { extend (a) } # [aapcs_on_arm] # [ppc_alias = __extendhfkf2] # [cfg (all (f16_enabled , f128_enabled))] pub extern "C" fn __extendhftf2 (a : f16) -> f128 { extend (a) } # [aapcs_on_arm] # [ppc_alias = __extendsfkf2] # [cfg (f128_enabled)] pub extern "C" fn __extendsftf2 (a : f32) -> f128 { extend (a) } # [aapcs_on_arm] # [ppc_alias = __extenddfkf2] # [cfg (f128_enabled)] pub extern "C" fn __extenddftf2 (a : f64) -> f128 { extend (a) } }
};
}
