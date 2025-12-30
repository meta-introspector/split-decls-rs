// Generated macro for strassen_sub_mul (function)
macro_rules! Depcrate_matmulstrassen_sub_mul {
() => {
// Module: crate::matmul
// Provides: {"strassen_sub_mul"}
// Dependencies: {}
fn strassen_sub_mul (b1 : & [f32] , b2 : & [f32] , a : & [f32]) -> Vec < f32 > { let mut dest = raw_buffer (a . len ()) ; let b = rtmp_sub (b1 , b2) ; matmul_strassen (a , & b [..] , & mut dest [..]) ; dest }
};
}
