// Generated macro for macro_29 (macro)
macro_rules! Depcrate_float_convmacro_29 {
() => {
// Module: crate::float::conv
// Provides: {"macro_29"}
// Dependencies: {}
intrinsics ! { # [arm_aeabi_alias = __aeabi_i2f] pub extern "C" fn __floatsisf (i : i32) -> f32 { int_to_float :: signed (i , int_to_float :: u32_to_f32_bits) } # [arm_aeabi_alias = __aeabi_i2d] pub extern "C" fn __floatsidf (i : i32) -> f64 { int_to_float :: signed (i , int_to_float :: u32_to_f64_bits) } # [arm_aeabi_alias = __aeabi_l2f] pub extern "C" fn __floatdisf (i : i64) -> f32 { int_to_float :: signed (i , int_to_float :: u64_to_f32_bits) } # [arm_aeabi_alias = __aeabi_l2d] pub extern "C" fn __floatdidf (i : i64) -> f64 { int_to_float :: signed (i , int_to_float :: u64_to_f64_bits) } # [cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floattisf (i : i128) -> f32 { int_to_float :: signed (i , int_to_float :: u128_to_f32_bits) } # [cfg_attr (target_os = "uefi" , unadjusted_on_win64)] pub extern "C" fn __floattidf (i : i128) -> f64 { int_to_float :: signed (i , int_to_float :: u128_to_f64_bits) } # [ppc_alias = __floatsikf] # [cfg (f128_enabled)] pub extern "C" fn __floatsitf (i : i32) -> f128 { int_to_float :: signed (i , int_to_float :: u32_to_f128_bits) } # [ppc_alias = __floatdikf] # [cfg (f128_enabled)] pub extern "C" fn __floatditf (i : i64) -> f128 { int_to_float :: signed (i , int_to_float :: u64_to_f128_bits) } # [ppc_alias = __floattikf] # [cfg (f128_enabled)] pub extern "C" fn __floattitf (i : i128) -> f128 { int_to_float :: signed (i , int_to_float :: u128_to_f128_bits) } }
};
}
