// Generated macro for CsCholesky (struct)
macro_rules! Depcrate_sparse_cs_matrix_choleskyCsCholesky {
() => {
// Module: crate::sparse::cs_matrix_cholesky
// Provides: {"CsCholesky"}
// Dependencies: {}
# [doc = " The cholesky decomposition of a column compressed sparse matrix."] pub struct CsCholesky < T : RealField , D : Dim > where DefaultAllocator : Allocator < D > , { original_p : Vec < usize > , original_i : Vec < usize > , l : CsMatrix < T , D , D > , u : CsMatrix < T , D , D > , ok : bool , work_x : OVector < T , D > , work_c : OVector < usize , D > , }
};
}
