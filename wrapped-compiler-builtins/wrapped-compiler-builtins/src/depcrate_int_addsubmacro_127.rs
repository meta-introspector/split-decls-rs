// Generated macro for macro_127 (macro)
macro_rules! Depcrate_int_addsubmacro_127 {
() => {
// Module: crate::int::addsub
// Provides: {"macro_127"}
// Dependencies: {}
intrinsics ! { pub extern "C" fn __rust_i128_add (a : i128 , b : i128) -> i128 { AddSub :: add (a , b) } pub extern "C" fn __rust_i128_addo (a : i128 , b : i128 , oflow : & mut i32) -> i128 { let (add , o) = a . addo (b) ; * oflow = o . into () ; add } pub extern "C" fn __rust_u128_add (a : u128 , b : u128) -> u128 { AddSub :: add (a , b) } pub extern "C" fn __rust_u128_addo (a : u128 , b : u128 , oflow : & mut i32) -> u128 { let (add , o) = a . addo (b) ; * oflow = o . into () ; add } pub extern "C" fn __rust_i128_sub (a : i128 , b : i128) -> i128 { AddSub :: sub (a , b) } pub extern "C" fn __rust_i128_subo (a : i128 , b : i128 , oflow : & mut i32) -> i128 { let (sub , o) = a . subo (b) ; * oflow = o . into () ; sub } pub extern "C" fn __rust_u128_sub (a : u128 , b : u128) -> u128 { AddSub :: sub (a , b) } pub extern "C" fn __rust_u128_subo (a : u128 , b : u128 , oflow : & mut i32) -> u128 { let (sub , o) = a . subo (b) ; * oflow = o . into () ; sub } }
};
}
