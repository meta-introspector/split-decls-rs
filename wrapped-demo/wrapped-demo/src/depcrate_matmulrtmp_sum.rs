// Generated macro for rtmp_sum (function)
macro_rules! Depcrate_matmulrtmp_sum {
() => {
// Module: crate::matmul
// Provides: {"rtmp_sum"}
// Dependencies: {}
fn rtmp_sum (a : & [f32] , b : & [f32]) -> Vec < f32 > { let mut tmp = raw_buffer (a . len ()) ; rcopy (a , & mut tmp [..]) ; rmatsum (b , & mut tmp [..]) ; tmp }
};
}
