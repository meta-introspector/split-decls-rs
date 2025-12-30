// Generated macro for macro_34 (macro)
macro_rules! Depcrate_float_convmacro_34 {
() => {
// Module: crate::float::conv
// Provides: {"macro_34"}
// Dependencies: {}
intrinsics ! { # [arm_aeabi_alias = __aeabi_f2iz] pub extern "C" fn __fixsfsi (f : f32) -> i32 { float_to_signed_int (f) } # [arm_aeabi_alias = __aeabi_f2lz] pub extern "C" fn __fixsfdi (f : f32) -> i64 { float_to_signed_int (f) } pub extern "C" fn __fixsfti (f : f32) -> i128 { float_to_signed_int (f) } # [arm_aeabi_alias = __aeabi_d2iz] pub extern "C" fn __fixdfsi (f : f64) -> i32 { float_to_signed_int (f) } # [arm_aeabi_alias = __aeabi_d2lz] pub extern "C" fn __fixdfdi (f : f64) -> i64 { float_to_signed_int (f) } pub extern "C" fn __fixdfti (f : f64) -> i128 { float_to_signed_int (f) } # [ppc_alias = __fixkfsi] # [cfg (f128_enabled)] pub extern "C" fn __fixtfsi (f : f128) -> i32 { float_to_signed_int (f) } # [ppc_alias = __fixkfdi] # [cfg (f128_enabled)] pub extern "C" fn __fixtfdi (f : f128) -> i64 { float_to_signed_int (f) } # [ppc_alias = __fixkfti] # [cfg (f128_enabled)] pub extern "C" fn __fixtfti (f : f128) -> i128 { float_to_signed_int (f) } }
};
}
