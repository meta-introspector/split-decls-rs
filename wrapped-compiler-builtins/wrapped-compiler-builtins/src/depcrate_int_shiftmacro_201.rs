// Generated macro for macro_201 (macro)
macro_rules! Depcrate_int_shiftmacro_201 {
() => {
// Module: crate::int::shift
// Provides: {"macro_201"}
// Dependencies: {}
intrinsics ! { # [maybe_use_optimized_c_shim] pub extern "C" fn __ashlsi3 (a : u32 , b : u32) -> u32 { a . ashl (b) } # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_llsl] pub extern "C" fn __ashldi3 (a : u64 , b : core :: ffi :: c_uint) -> u64 { a . ashl (b as u32) } pub extern "C" fn __ashlti3 (a : u128 , b : u32) -> u128 { a . ashl (b) } # [maybe_use_optimized_c_shim] pub extern "C" fn __ashrsi3 (a : i32 , b : u32) -> i32 { a . ashr (b) } # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_lasr] pub extern "C" fn __ashrdi3 (a : i64 , b : core :: ffi :: c_uint) -> i64 { a . ashr (b as u32) } pub extern "C" fn __ashrti3 (a : i128 , b : u32) -> i128 { a . ashr (b) } # [maybe_use_optimized_c_shim] pub extern "C" fn __lshrsi3 (a : u32 , b : u32) -> u32 { a . lshr (b) } # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_llsr] pub extern "C" fn __lshrdi3 (a : u64 , b : core :: ffi :: c_uint) -> u64 { a . lshr (b as u32) } pub extern "C" fn __lshrti3 (a : u128 , b : u32) -> u128 { a . lshr (b) } }
};
}
