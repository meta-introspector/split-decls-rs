// Generated macro for matmul_strassen (function)
macro_rules! Depcrate_matmulmatmul_strassen {
() => {
// Module: crate::matmul
// Provides: {"matmul_strassen"}
// Dependencies: {}
pub fn matmul_strassen (a : & [f32] , b : & [f32] , dest : & mut [f32]) { if a . len () <= MULT_CHUNK { seq_matmulz (a , b , dest) ; return ; } let (a11 , a12 , a21 , a22) = quarter_chunks (a) ; let (b11 , b12 , b21 , b22) = quarter_chunks (b) ; let (m1 , m2 , m3 , m4 , m5 , m6 , m7 , _) = join8 (| | strassen_add2_mul (a11 , a22 , b11 , b22) , | | strassen_add_mul (a21 , a22 , b11) , | | strassen_sub_mul (b12 , b22 , a11) , | | strassen_sub_mul (b21 , b11 , a22) , | | strassen_add_mul (a11 , a12 , b22) , | | strassen_sub_add_mul (a21 , a11 , b11 , b12) , | | strassen_sub_add_mul (a12 , a22 , b21 , b22) , | | () ,) ; let (c11 , c12 , c21 , c22) = quarter_chunks_mut (dest) ; join4 (| | strassen_sum_sub (& m1 [..] , & m4 [..] , & m7 [..] , & m5 [..] , c11) , | | strassen_sum (& m3 [..] , & m5 [..] , c12) , | | strassen_sum (& m2 [..] , & m4 [..] , c21) , | | strassen_sum_sub (& m1 [..] , & m3 [..] , & m6 [..] , & m2 [..] , c22) ,) ; }
};
}
