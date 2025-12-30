// Generated macro for strassen_sum_sub (function)
macro_rules! Depcrate_matmulstrassen_sum_sub {
() => {
// Module: crate::matmul
// Provides: {"strassen_sum_sub"}
// Dependencies: {}
fn strassen_sum_sub (a : & [f32] , b : & [f32] , c : & [f32] , s : & [f32] , dest : & mut [f32]) { rcopy (a , dest) ; rmatsum (b , dest) ; rmatsum (c , dest) ; rmatsub (s , dest) ; }
};
}
