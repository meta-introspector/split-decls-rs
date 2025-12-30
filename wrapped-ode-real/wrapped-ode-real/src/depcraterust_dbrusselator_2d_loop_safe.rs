// Generated macro for rust_dbrusselator_2d_loop_safe (function)
macro_rules! Depcraterust_dbrusselator_2d_loop_safe {
() => {
// Module: crate
// Provides: {"rust_dbrusselator_2d_loop_safe"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn rust_dbrusselator_2d_loop_safe (adjoint : * mut StateType , x : * const StateType , dx : * mut StateType , p : * const [f64 ; 3] , dp : * mut [f64 ; 3] , t : f64) { let x : & StateType = unsafe { & * x } ; let dx : & mut StateType = unsafe { & mut * dx } ; let adjoint : & mut StateType = unsafe { & mut * adjoint } ; let p : & [f64 ; 3] = unsafe { & * p } ; let dp : & mut [f64 ; 3] = unsafe { & mut * dp } ; assert ! (p [0] == 3.4) ; assert ! (p [1] == 1.) ; assert ! (p [2] == 10.) ; assert ! (t == 2.1) ; let ([dx1 , dx2] , []) : (& mut [[f64 ; N * N]] , & mut [f64]) = dx . as_chunks_mut () else { unreachable ! () } ; let ([dadj1 , dadj2] , []) : (& mut [[f64 ; N * N]] , & mut [f64]) = adjoint . as_chunks_mut () else { unreachable ! () } ; let ([x1 , x2] , []) : (& [[f64 ; N * N]] , & [f64]) = x . as_chunks () else { unreachable ! () } ; let mut null1 = [0. ; 1 * N * N] ; let mut null2 = [0. ; 1 * N * N] ; safe :: dbrusselator_2d_loop (& mut null1 , dadj1 , & mut null2 , dadj2 , x1 , dx1 , x2 , dx2 , p , dp , t) ; return ; }
};
}
