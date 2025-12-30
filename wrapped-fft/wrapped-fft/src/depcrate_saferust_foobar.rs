// Generated macro for rust_foobar (function)
macro_rules! Depcrate_saferust_foobar {
() => {
// Module: crate::safe
// Provides: {"rust_foobar"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_foobar (n : usize , data : * mut f64) { let data = unsafe { slice :: from_raw_parts_mut (data , n * 2) } ; foobar (data) ; }
};
}
