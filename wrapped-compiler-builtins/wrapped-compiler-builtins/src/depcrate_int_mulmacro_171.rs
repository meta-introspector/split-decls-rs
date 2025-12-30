// Generated macro for macro_171 (macro)
macro_rules! Depcrate_int_mulmacro_171 {
() => {
// Module: crate::int::mul
// Provides: {"macro_171"}
// Dependencies: {}
intrinsics ! { # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_lmul] # [cfg (any (not (any (target_arch = "riscv32" , target_arch = "riscv64")) , target_feature = "m"))] pub extern "C" fn __muldi3 (a : u64 , b : u64) -> u64 { a . mul (b) } pub extern "C" fn __multi3 (a : i128 , b : i128) -> i128 { a . mul (b) } pub extern "C" fn __mulosi4 (a : i32 , b : i32 , oflow : & mut i32) -> i32 { let (mul , o) = i32_overflowing_mul (a , b) ; * oflow = o as i32 ; mul } pub extern "C" fn __mulodi4 (a : i64 , b : i64 , oflow : & mut i32) -> i64 { let (mul , o) = i64_overflowing_mul (a , b) ; * oflow = o as i32 ; mul } # [unadjusted_on_win64] pub extern "C" fn __muloti4 (a : i128 , b : i128 , oflow : & mut i32) -> i128 { let (mul , o) = i128_overflowing_mul (a , b) ; * oflow = o as i32 ; mul } pub extern "C" fn __rust_i128_mulo (a : i128 , b : i128 , oflow : & mut i32) -> i128 { let (mul , o) = i128_overflowing_mul (a , b) ; * oflow = o . into () ; mul } pub extern "C" fn __rust_u128_mulo (a : u128 , b : u128 , oflow : & mut i32) -> u128 { let (mul , o) = a . mulo (b) ; * oflow = o . into () ; mul } }
};
}
