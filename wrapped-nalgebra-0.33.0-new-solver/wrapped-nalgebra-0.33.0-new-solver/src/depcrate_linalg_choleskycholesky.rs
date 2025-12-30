// Generated macro for Cholesky (struct)
macro_rules! Depcrate_linalg_choleskyCholesky {
() => {
// Module: crate::linalg::cholesky
// Provides: {"Cholesky"}
// Dependencies: {}
# [doc = " The Cholesky decomposition of a symmetric-definite-positive matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D>,
         OMatrix<T, D, D>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D>,
         OMatrix<T, D, D>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct Cholesky < T : SimdComplexField , D : Dim > where DefaultAllocator : Allocator < D , D > , { chol : OMatrix < T , D , D > , }
};
}
