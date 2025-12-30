// Generated macro for f64_dot (function)
macro_rules! Depcrate_slicef64_dot {
() => {
// Module: crate::slice
// Provides: {"f64_dot"}
// Dependencies: {}
# [cfg (test)] pub fn f64_dot (xs : & [f64] , ys : & [f64]) -> f64 { let mut sum = [0. ; 8] ; zip_unroll_8 (xs , ys , | i , x , y | sum [i] += x * y) ; sum [0] += sum [4] ; sum [1] += sum [5] ; sum [2] += sum [6] ; sum [3] += sum [7] ; sum [0] += sum [2] ; sum [1] += sum [3] ; sum [0] + sum [1] }
};
}
