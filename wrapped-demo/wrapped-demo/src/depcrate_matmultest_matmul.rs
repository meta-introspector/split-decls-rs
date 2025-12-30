// Generated macro for test_matmul (function)
macro_rules! Depcrate_matmultest_matmul {
() => {
// Module: crate::matmul
// Provides: {"test_matmul"}
// Dependencies: {}
# [test] fn test_matmul () { let a : Vec < f32 > = vec ! [1.0 , 2.0 , 3.0 , 4.0] ; let b : Vec < f32 > = vec ! [5.0 , 6.0 , 7.0 , 8.0] ; let mul : Vec < f32 > = vec ! [19.0 , 22.0 , 43.0 , 50.0] ; let mut dest = vec ! [0f32 ; 4] ; matmulz (& a [..] , & b [..] , & mut dest [..]) ; assert_eq ! (mul , dest) ; seq_matmulz (& a [..] , & b [..] , & mut dest [..]) ; assert_eq ! (mul , dest) ; matmul_strassen (& a [..] , & b [..] , & mut dest [..]) ; assert_eq ! (mul , dest) ; let n = 1 << 14 ; assert ! (n > MULT_CHUNK) ; let a : Vec < f32 > = (0 .. n) . map (| i | (i % 101) as f32) . collect () ; let b : Vec < f32 > = (0 .. n) . map (| i | (i % 101 + 7) as f32) . collect () ; let mut seqmul = vec ! [0f32 ; n] ; seq_matmulz (& a [..] , & b [..] , & mut seqmul [..]) ; let mut rmul = vec ! [0f32 ; n] ; matmulz (& a [..] , & b [..] , & mut rmul [..]) ; assert_eq ! (rmul , seqmul) ; for d in rmul . iter_mut () { * d = 0.0 ; } matmul_strassen (& a [..] , & b [..] , & mut rmul [..]) ; assert_eq ! (rmul , seqmul) ; }
};
}
