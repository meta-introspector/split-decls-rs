// Generated macro for quarter_chunks_mut (function)
macro_rules! Depcrate_matmulquarter_chunks_mut {
() => {
// Module: crate::matmul
// Provides: {"quarter_chunks_mut"}
// Dependencies: {}
fn quarter_chunks_mut (v : & mut [f32]) -> (& mut [f32] , & mut [f32] , & mut [f32] , & mut [f32]) { let mid = v . len () / 2 ; let quarter = mid / 2 ; let (left , right) = v . split_at_mut (mid) ; let (a , b) = left . split_at_mut (quarter) ; let (c , d) = right . split_at_mut (quarter) ; (a , b , c , d) }
};
}
