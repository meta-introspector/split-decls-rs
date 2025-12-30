// Generated macro for onenorm_matrix_power_nonm (function)
macro_rules! Depcrate_linalg_exponenorm_matrix_power_nonm {
() => {
// Module: crate::linalg::exp
// Provides: {"onenorm_matrix_power_nonm"}
// Dependencies: {}
# [doc = " Compute the 1-norm of a non-negative integer power of a non-negative matrix."] fn onenorm_matrix_power_nonm < T , D > (a : & OMatrix < T , D , D > , p : usize) -> T where T : RealField , D : Dim , DefaultAllocator : Allocator < D , D > + Allocator < D > , { let nrows = a . shape_generic () . 0 ; let mut v = crate :: OVector :: < T , D > :: repeat_generic (nrows , Const :: < 1 > , convert (1.0)) ; let m = a . transpose () ; for _ in 0 .. p { v = & m * v ; } v . max () }
};
}
