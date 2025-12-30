// Generated macro for macro_33 (macro)
macro_rules! Depcrate_float_convmacro_33 {
() => {
// Module: crate::float::conv
// Provides: {"macro_33"}
// Dependencies: {}
intrinsics ! { # [arm_aeabi_alias = __aeabi_f2uiz] pub extern "C" fn __fixunssfsi (f : f32) -> u32 { float_to_unsigned_int (f) } # [arm_aeabi_alias = __aeabi_f2ulz] pub extern "C" fn __fixunssfdi (f : f32) -> u64 { float_to_unsigned_int (f) } pub extern "C" fn __fixunssfti (f : f32) -> u128 { float_to_unsigned_int (f) } # [arm_aeabi_alias = __aeabi_d2uiz] pub extern "C" fn __fixunsdfsi (f : f64) -> u32 { float_to_unsigned_int (f) } # [arm_aeabi_alias = __aeabi_d2ulz] pub extern "C" fn __fixunsdfdi (f : f64) -> u64 { float_to_unsigned_int (f) } pub extern "C" fn __fixunsdfti (f : f64) -> u128 { float_to_unsigned_int (f) } # [ppc_alias = __fixunskfsi] # [cfg (f128_enabled)] pub extern "C" fn __fixunstfsi (f : f128) -> u32 { float_to_unsigned_int (f) } # [ppc_alias = __fixunskfdi] # [cfg (f128_enabled)] pub extern "C" fn __fixunstfdi (f : f128) -> u64 { float_to_unsigned_int (f) } # [ppc_alias = __fixunskfti] # [cfg (f128_enabled)] pub extern "C" fn __fixunstfti (f : f128) -> u128 { float_to_unsigned_int (f) } }
};
}
