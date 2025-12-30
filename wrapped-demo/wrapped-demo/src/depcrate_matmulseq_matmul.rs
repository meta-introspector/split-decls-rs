// Generated macro for seq_matmul (function)
macro_rules! Depcrate_matmulseq_matmul {
() => {
// Module: crate::matmul
// Provides: {"seq_matmul"}
// Dependencies: {}
pub fn seq_matmul (a : & [f32] , b : & [f32] , dest : & mut [f32]) { for d in dest . iter_mut () { * d = 0.0 ; } let bits = dest . len () . trailing_zeros () / 2 ; let n = 1 << bits ; for i in 0 .. n { for j in 0 .. n { let mut sum = 0.0 ; for k in 0 .. n { sum += unsafe { a . get_unchecked ((i << bits) | k) * b . get_unchecked ((k << bits) | j) } ; } dest [(i << bits) | j] = sum ; } } }
};
}
