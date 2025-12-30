// Generated macro for lorenz (function)
macro_rules! Depcrate_safelorenz {
() => {
// Module: crate::safe
// Provides: {"lorenz"}
// Dependencies: {}
pub fn lorenz (x : & StateType , dxdt : & mut StateType , t : f64) { let p = [3.4 , 1. , 10.] ; let (tmp1 , tmp2) = dxdt . split_at_mut (N * N) ; let mut dxdt1 : [f64 ; N * N] = tmp1 . try_into () . unwrap () ; let mut dxdt2 : [f64 ; N * N] = tmp2 . try_into () . unwrap () ; let (tmp1 , tmp2) = x . split_at (N * N) ; let u : [f64 ; N * N] = tmp1 . try_into () . unwrap () ; let v : [f64 ; N * N] = tmp2 . try_into () . unwrap () ; brusselator_2d_loop (& mut dxdt1 , & mut dxdt2 , & u , & v , & p , t) ; }
};
}
