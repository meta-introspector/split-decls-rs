// Generated macro for lorenz (function)
macro_rules! Depcrate_unsflorenz {
() => {
// Module: crate::unsf
// Provides: {"lorenz"}
// Dependencies: {}
pub unsafe fn lorenz (x : * const StateType , dxdt : * mut StateType , t : f64) { let p = [3.4 , 1. , 10.] ; let x = x as * const f64 ; let dxdt = dxdt as * mut f64 ; let dxdt1 : * mut f64 = dxdt as * mut f64 ; let dxdt2 : * mut f64 = unsafe { dxdt . add (N * N) } as * mut f64 ; let u : * const f64 = x as * const f64 ; let v : * const f64 = unsafe { x . add (N * N) } as * const f64 ; unsafe { brusselator_2d_loop_unsf (dxdt1 as * mut f64 , dxdt2 as * mut f64 , u as * const f64 , v as * const f64 , p . as_ptr () , t) } ; }
};
}
