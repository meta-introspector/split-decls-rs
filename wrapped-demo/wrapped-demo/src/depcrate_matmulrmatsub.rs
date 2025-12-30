// Generated macro for rmatsub (function)
macro_rules! Depcrate_matmulrmatsub {
() => {
// Module: crate::matmul
// Provides: {"rmatsub"}
// Dependencies: {}
fn rmatsub (src : & [f32] , dest : & mut [f32]) { dest . par_iter_mut () . zip (src . par_iter ()) . for_each (| (d , s) | * d -= * s) ; }
};
}
