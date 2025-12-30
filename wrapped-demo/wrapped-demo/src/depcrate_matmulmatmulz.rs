// Generated macro for matmulz (function)
macro_rules! Depcrate_matmulmatmulz {
() => {
// Module: crate::matmul
// Provides: {"matmulz"}
// Dependencies: {}
pub fn matmulz (a : & [f32] , b : & [f32] , dest : & mut [f32]) { if a . len () <= MULT_CHUNK { seq_matmulz (a , b , dest) ; return ; } let mut tmp = raw_buffer (dest . len ()) ; let (a1 , a2 , a3 , a4) = quarter_chunks (a) ; let (b1 , b2 , b3 , b4) = quarter_chunks (b) ; { let (d1 , d2 , d3 , d4) = quarter_chunks_mut (dest) ; let (t1 , t2 , t3 , t4) = quarter_chunks_mut (& mut tmp [..]) ; join8 (| | matmulz (a1 , b1 , d1) , | | matmulz (a1 , b2 , d2) , | | matmulz (a3 , b1 , d3) , | | matmulz (a3 , b2 , d4) , | | matmulz (a2 , b3 , t1) , | | matmulz (a2 , b4 , t2) , | | matmulz (a4 , b3 , t3) , | | matmulz (a4 , b4 , t4) ,) ; } rmatsum (tmp . as_mut () , dest) ; }
};
}
