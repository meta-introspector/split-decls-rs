// Generated macro for strassen_add2_mul (function)
macro_rules! Depcrate_matmulstrassen_add2_mul {
() => {
// Module: crate::matmul
// Provides: {"strassen_add2_mul"}
// Dependencies: {}
fn strassen_add2_mul (a1 : & [f32] , a2 : & [f32] , b1 : & [f32] , b2 : & [f32]) -> Vec < f32 > { let mut dest = raw_buffer (a1 . len ()) ; let (a , b) = rayon :: join (| | rtmp_sum (a1 , a2) , | | rtmp_sum (b1 , b2)) ; matmul_strassen (& a [..] , & b [..] , & mut dest [..]) ; dest }
};
}
