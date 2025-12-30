// Generated macro for free (function)
macro_rules! Depcrate_stringfree {
() => {
// Module: crate::string
// Provides: {"free"}
// Dependencies: {}
# [inline] # [cfg (any (boringssl , awslc))] unsafe fn free (buf : * mut c_char) { ffi :: CRYPTO_free (buf as * mut c_void , concat ! (file ! () , "\0") . as_ptr () as * const c_char , line ! () as :: libc :: c_int ,) ; }
};
}
