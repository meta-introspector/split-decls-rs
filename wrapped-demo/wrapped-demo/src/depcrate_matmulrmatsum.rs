// Generated macro for rmatsum (function)
macro_rules! Depcrate_matmulrmatsum {
() => {
// Module: crate::matmul
// Provides: {"rmatsum"}
// Dependencies: {}
fn rmatsum (src : & [f32] , dest : & mut [f32]) { dest . par_iter_mut () . zip (src . par_iter ()) . for_each (| (d , s) | * d += * s) ; }
};
}
