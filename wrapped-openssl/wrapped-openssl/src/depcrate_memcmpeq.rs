// Generated macro for eq (function)
macro_rules! Depcrate_memcmpeq {
() => {
// Module: crate::memcmp
// Provides: {"eq"}
// Dependencies: {}
# [doc = " Returns `true` iff `a` and `b` contain the same bytes."] # [doc = ""] # [doc = " This operation takes an amount of time dependent on the length of the two"] # [doc = " arrays given, but is independent of the contents of a and b."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function will panic the current task if `a` and `b` do not have the same"] # [doc = " length."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " To perform a constant-time comparison of two arrays of the same length but different"] # [doc = " values:"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::memcmp::eq;"] # [doc = ""] # [doc = " // We want to compare `a` to `b` and `c`, without giving"] # [doc = " // away through timing analysis that `c` is more similar to `a`"] # [doc = " // than `b`."] # [doc = " let a = [0, 0, 0];"] # [doc = " let b = [1, 1, 1];"] # [doc = " let c = [0, 0, 1];"] # [doc = ""] # [doc = " // These statements will execute in the same amount of time."] # [doc = " assert!(!eq(&a, &b));"] # [doc = " assert!(!eq(&a, &c));"] # [doc = " ```"] # [corresponds (CRYPTO_memcmp)] pub fn eq (a : & [u8] , b : & [u8]) -> bool { assert ! (a . len () == b . len ()) ; let ret = unsafe { ffi :: CRYPTO_memcmp (a . as_ptr () as * const _ , b . as_ptr () as * const _ , a . len () as size_t ,) } ; ret == 0 }
};
}
