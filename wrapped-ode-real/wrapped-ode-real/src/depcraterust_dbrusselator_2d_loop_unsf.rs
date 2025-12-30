// Generated macro for rust_dbrusselator_2d_loop_unsf (function)
macro_rules! Depcraterust_dbrusselator_2d_loop_unsf {
() => {
// Module: crate
// Provides: {"rust_dbrusselator_2d_loop_unsf"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_dbrusselator_2d_loop_unsf (adjoint : * mut StateType , x : * const StateType , dx : * mut StateType , p : * const [f64 ; 3] , dp : * mut [f64 ; 3] , t : f64) { let mut null1 = [0. ; 1 * N * N] ; let mut null2 = [0. ; 1 * N * N] ; let dx1 : * mut f64 = dx . as_mut_ptr () ; let dx2 : * mut f64 = unsafe { dx . as_mut_ptr () . add (N * N) } ; let dadj1 : * mut f64 = adjoint . as_mut_ptr () ; let dadj2 : * mut f64 = unsafe { adjoint . as_mut_ptr () . add (N * N) } ; let x1 : * const f64 = x . as_ptr () ; let x2 : * const f64 = unsafe { x . as_ptr () . add (N * N) } ; unsafe { unsf :: dbrusselator_2d_loop_unsf (null1 . as_mut_ptr () , dadj1 , null2 . as_mut_ptr () , dadj2 , x1 , dx1 , x2 , dx2 , p as * mut f64 , dp as * mut f64 , t) } ; }
};
}
