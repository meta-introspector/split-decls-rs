// Generated macro for macro_28 (macro)
macro_rules! Depcrate_float_convmacro_28 {
() => {
// Module: crate::float::conv
// Provides: {"macro_28"}
// Dependencies: {}
intrinsics ! { # [arm_aeabi_alias = __aeabi_ui2f] pub extern "C" fn __floatunsisf (i : u32) -> f32 { f32 :: from_bits (int_to_float :: u32_to_f32_bits (i)) } # [arm_aeabi_alias = __aeabi_ui2d] pub extern "C" fn __floatunsidf (i : u32) -> f64 { f64 :: from_bits (int_to_float :: u32_to_f64_bits (i)) } # [arm_aeabi_alias = __aeabi_ul2f] pub extern "C" fn __floatundisf (i : u64) -> f32 { f32 :: from_bits (int_to_float :: u64_to_f32_bits (i)) } # [arm_aeabi_alias = __aeabi_ul2d] pub extern "C" fn __floatundidf (i : u64) -> f64 { f64 :: from_bits (int_to_float :: u64_to_f64_bits (i)) } # [cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floatuntisf (i : u128) -> f32 { f32 :: from_bits (int_to_float :: u128_to_f32_bits (i)) } # [cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floatuntidf (i : u128) -> f64 { f64 :: from_bits (int_to_float :: u128_to_f64_bits (i)) } # [ppc_alias = __floatunsikf] # [cfg (f128_enabled)] pub extern "C" fn __floatunsitf (i : u32) -> f128 { f128 :: from_bits (int_to_float :: u32_to_f128_bits (i)) } # [ppc_alias = __floatundikf] # [cfg (f128_enabled)] pub extern "C" fn __floatunditf (i : u64) -> f128 { f128 :: from_bits (int_to_float :: u64_to_f128_bits (i)) } # [ppc_alias = __floatuntikf] # [cfg (f128_enabled)] pub extern "C" fn __floatuntitf (i : u128) -> f128 { f128 :: from_bits (int_to_float :: u128_to_f128_bits (i)) } }
};
}
