// Generated macro for strassen_add_mul (function)
macro_rules! Depcrate_matmulstrassen_add_mul {
() => {
// Module: crate::matmul
// Provides: {"strassen_add_mul"}
// Dependencies: {}
fn strassen_add_mul (a1 : & [f32] , a2 : & [f32] , b : & [f32]) -> Vec < f32 > { let mut dest = raw_buffer (a1 . len ()) ; let a = rtmp_sum (a1 , a2) ; matmul_strassen (& a [..] , b , & mut dest [..]) ; dest }
};
}
