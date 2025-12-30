// Generated macro for impl_3091 (impl)
macro_rules! Depcrate_sparse_cs_matrix_conversionimpl_3091 {
() => {
// Module: crate::sparse::cs_matrix_conversion
// Provides: {"impl_3091"}
// Dependencies: {}
impl < 'a , T : Scalar + Zero + ClosedAddAssign > CsMatrix < T > { # [doc = " Creates a column-compressed sparse matrix from a sparse matrix in triplet form."] pub fn from_triplet (nrows : usize , ncols : usize , irows : & [usize] , icols : & [usize] , vals : & [T] ,) -> Self { Self :: from_triplet_generic (Dyn (nrows) , Dyn (ncols) , irows , icols , vals) } }
};
}
