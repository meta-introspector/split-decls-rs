// Generated macro for compute_reproj_error (function)
macro_rules! Depcrate_safecompute_reproj_error {
() => {
// Module: crate::safe
// Provides: {"compute_reproj_error"}
// Dependencies: {}
# [autodiff (dcompute_reproj_error , Reverse , Duplicated , Duplicated , Duplicated , Const , DuplicatedOnly)] pub fn compute_reproj_error (cam : * const [f64 ; 11] , x : * const [f64 ; 3] , w : * const [f64 ; 1] , feat : * const [f64 ; 2] , err : * mut [f64 ; 2] ,) { let cam = unsafe { & * cam } ; let w = unsafe { * (* w) . get_unchecked (0) } ; let x = unsafe { & * x } ; let feat = unsafe { & * feat } ; let err = unsafe { & mut * err } ; let mut proj = [0. ; 2] ; project (cam , x , & mut proj) ; err [0] = w * (proj [0] - feat [0]) ; err [1] = w * (proj [1] - feat [1]) ; }
};
}
