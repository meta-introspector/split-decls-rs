// Generated macro for strassen_sum (function)
macro_rules! Depcrate_matmulstrassen_sum {
() => {
// Module: crate::matmul
// Provides: {"strassen_sum"}
// Dependencies: {}
fn strassen_sum (a : & [f32] , b : & [f32] , dest : & mut [f32]) { rcopy (a , dest) ; rmatsum (b , dest) ; }
};
}
