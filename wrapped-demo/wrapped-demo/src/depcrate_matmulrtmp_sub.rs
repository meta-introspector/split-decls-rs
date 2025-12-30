// Generated macro for rtmp_sub (function)
macro_rules! Depcrate_matmulrtmp_sub {
() => {
// Module: crate::matmul
// Provides: {"rtmp_sub"}
// Dependencies: {}
fn rtmp_sub (a : & [f32] , b : & [f32]) -> Vec < f32 > { let mut tmp = raw_buffer (a . len ()) ; rcopy (a , & mut tmp [..]) ; rmatsub (b , & mut tmp [..]) ; tmp }
};
}
