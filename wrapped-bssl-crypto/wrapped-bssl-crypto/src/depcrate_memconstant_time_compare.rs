// Generated macro for constant_time_compare (function)
macro_rules! Depcrate_memconstant_time_compare {
() => {
// Module: crate::mem
// Provides: {"constant_time_compare"}
// Dependencies: {}
# [doc = " Returns true iff `a` and `b` contain the same bytes. It takes an amount of time dependent on the"] # [doc = " lengths, but independent of the contents of the slices `a` and `b`. The return type is a `bool`,"] # [doc = " since unlike `memcmp` in C this function cannot be used to put elements into a defined order."] pub fn constant_time_compare (a : & [u8] , b : & [u8]) -> bool { if a . len () != b . len () { return false ; } if a . is_empty () { return true ; } let result = unsafe { bssl_sys :: CRYPTO_memcmp (a . as_ffi_void_ptr () , b . as_ffi_void_ptr () , a . len ()) } ; result == 0 }
};
}
