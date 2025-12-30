// Generated macro for compute (function)
macro_rules! Depcrate_laplacecompute {
() => {
// Module: crate::laplace
// Provides: {"compute"}
// Dependencies: {}
fn compute (matrix : & mut [f64] , size_x : usize , size_y : usize , iterations : usize) -> f64 { let mut clone = matrix . to_vec () ; let mut current = matrix ; let mut next = & mut clone [..] ; for _ in 0 .. iterations { iteration (current , next , size_x , size_y) ; mem :: swap (& mut current , & mut next) ; } get_residual (current , size_x , size_y) }
};
}
