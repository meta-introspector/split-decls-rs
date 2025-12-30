// Generated macro for quarter_chunks (function)
macro_rules! Depcrate_matmulquarter_chunks {
() => {
// Module: crate::matmul
// Provides: {"quarter_chunks"}
// Dependencies: {}
fn quarter_chunks (v : & [f32]) -> (& [f32] , & [f32] , & [f32] , & [f32]) { let mid = v . len () / 2 ; let quarter = mid / 2 ; let (left , right) = v . split_at (mid) ; let (a , b) = left . split_at (quarter) ; let (c , d) = right . split_at (quarter) ; (a , b , c , d) }
};
}
