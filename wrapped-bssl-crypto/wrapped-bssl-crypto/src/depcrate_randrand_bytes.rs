// Generated macro for rand_bytes (function)
macro_rules! Depcrate_randrand_bytes {
() => {
// Module: crate::rand
// Provides: {"rand_bytes"}
// Dependencies: {}
# [doc = " Fills `buf` with random bytes."] pub fn rand_bytes (buf : & mut [u8]) { let ret = unsafe { bssl_sys :: RAND_bytes (buf . as_mut_ffi_ptr () , buf . len ()) } ; debug_assert ! (ret == 1) ; }
};
}
