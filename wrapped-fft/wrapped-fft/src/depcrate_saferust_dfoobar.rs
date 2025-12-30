// Generated macro for rust_dfoobar (function)
macro_rules! Depcrate_saferust_dfoobar {
() => {
// Module: crate::safe
// Provides: {"rust_dfoobar"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_dfoobar (n : usize , data : * mut f64 , ddata : * mut f64) { let (data , ddata) = unsafe { (slice :: from_raw_parts_mut (data , n * 2) , slice :: from_raw_parts_mut (ddata , n * 2) ,) } ; unsafe { dfoobar (data , ddata) } ; }
};
}
