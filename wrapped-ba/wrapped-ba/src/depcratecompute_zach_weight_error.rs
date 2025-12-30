// Generated macro for compute_zach_weight_error (function)
macro_rules! Depcratecompute_zach_weight_error {
() => {
// Module: crate
// Provides: {"compute_zach_weight_error"}
// Dependencies: {}
# [autodiff (dcompute_zach_weight_error , Reverse , Duplicated , Duplicated)] pub fn compute_zach_weight_error (w : * const f64 , err : * mut f64) { let w = unsafe { * w } ; unsafe { * err = 1. - w * w ; } }
};
}
