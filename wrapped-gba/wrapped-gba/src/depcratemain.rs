// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [no_mangle] extern "C" fn main () -> ! { macro_rules ! test_atomic_int { ($ int_type : ident) => { paste :: paste ! { fn [< test_atomic_ $ int_type >] () { __test_atomic_int ! ([< Atomic $ int_type : camel >] , $ int_type) ; } print ! ("test test_atomic_{} ... " , stringify ! ($ int_type)) ; [< test_atomic_ $ int_type >] () ; println ! ("ok") ; } } ; } macro_rules ! test_atomic_float { ($ float_type : ident) => { paste :: paste ! { fn [< test_atomic_ $ float_type >] () { __test_atomic_float ! ([< Atomic $ float_type : camel >] , $ float_type) ; } print ! ("test test_atomic_{} ... " , stringify ! ($ float_type)) ; [< test_atomic_ $ float_type >] () ; println ! ("ok") ; } } ; } macro_rules ! test_atomic_bool { () => { fn test_atomic_bool () { __test_atomic_bool ! (AtomicBool) ; } print ! ("test test_atomic_bool ... ") ; test_atomic_bool () ; println ! ("ok") ; } ; } macro_rules ! test_atomic_ptr { () => { fn test_atomic_ptr () { __test_atomic_ptr ! (AtomicPtr < u8 >) ; } print ! ("test test_atomic_ptr ... ") ; test_atomic_ptr () ; println ! ("ok") ; } ; } println ! ("starting tests...") ; hint :: spin_loop () ; test_atomic_bool ! () ; test_atomic_ptr ! () ; test_atomic_int ! (isize) ; test_atomic_int ! (usize) ; test_atomic_int ! (i8) ; test_atomic_int ! (u8) ; test_atomic_int ! (i16) ; test_atomic_int ! (u16) ; test_atomic_int ! (i32) ; test_atomic_int ! (u32) ; test_atomic_int ! (i64) ; test_atomic_int ! (u64) ; test_atomic_int ! (i128) ; test_atomic_int ! (u128) ; test_atomic_float ! (f32) ; test_atomic_float ! (f64) ; println ! ("Tests finished successfully") ; # [allow (clippy :: empty_loop)] loop { } }
};
}
