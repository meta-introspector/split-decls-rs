// Generated macro for macro_9 (macro)
macro_rules! Depcrate_float_addmacro_9 {
() => {
// Module: crate::float::add
// Provides: {"macro_9"}
// Dependencies: {}
intrinsics ! { # [cfg (f16_enabled)] pub extern "C" fn __addhf3 (a : f16 , b : f16) -> f16 { add (a , b) } # [aapcs_on_arm] # [arm_aeabi_alias = __aeabi_fadd] pub extern "C" fn __addsf3 (a : f32 , b : f32) -> f32 { add (a , b) } # [aapcs_on_arm] # [arm_aeabi_alias = __aeabi_dadd] pub extern "C" fn __adddf3 (a : f64 , b : f64) -> f64 { add (a , b) } # [ppc_alias = __addkf3] # [cfg (f128_enabled)] pub extern "C" fn __addtf3 (a : f128 , b : f128) -> f128 { add (a , b) } }
};
}
