// Generated macro for rcopy (function)
macro_rules! Depcrate_matmulrcopy {
() => {
// Module: crate::matmul
// Provides: {"rcopy"}
// Dependencies: {}
fn rcopy (src : & [f32] , dest : & mut [f32]) { if dest . len () <= LINEAR_CHUNK { dest . copy_from_slice (src) ; return ; } let mid = dest . len () / 2 ; let (s1 , s2) = src . split_at (mid) ; let (d1 , d2) = dest . split_at_mut (mid) ; rayon :: join (| | rcopy (s1 , d1) , | | rcopy (s2 , d2)) ; }
};
}
