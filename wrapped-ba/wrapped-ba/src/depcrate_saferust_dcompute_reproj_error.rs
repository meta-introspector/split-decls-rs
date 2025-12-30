// Generated macro for rust_dcompute_reproj_error (function)
macro_rules! Depcrate_saferust_dcompute_reproj_error {
() => {
// Module: crate::safe
// Provides: {"rust_dcompute_reproj_error"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_dcompute_reproj_error (cam : * const [f64 ; 11] , dcam : * mut [f64 ; 11] , x : * const [f64 ; 3] , dx : * mut [f64 ; 3] , w : * const [f64 ; 1] , wb : * mut [f64 ; 1] , feat : * const [f64 ; 2] , err : * mut [f64 ; 2] , derr : * mut [f64 ; 2] ,) { unsafe { dcompute_reproj_error (cam , dcam , x , dx , w , wb , feat , err , derr) } ; }
};
}
