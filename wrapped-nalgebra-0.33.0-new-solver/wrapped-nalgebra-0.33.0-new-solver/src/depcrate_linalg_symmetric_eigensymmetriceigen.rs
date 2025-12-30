// Generated macro for SymmetricEigen (struct)
macro_rules! Depcrate_linalg_symmetric_eigenSymmetricEigen {
() => {
// Module: crate::linalg::symmetric_eigen
// Provides: {"SymmetricEigen"}
// Dependencies: {}
# [doc = " Eigendecomposition of a symmetric matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<D>,
         OVector<T::RealField, D>: Serialize,
         OMatrix<T, D, D>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<D>,
         OVector<T::RealField, D>: Deserialize<'de>,
         OMatrix<T, D, D>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct SymmetricEigen < T : ComplexField , D : Dim > where DefaultAllocator : Allocator < D , D > + Allocator < D > , { # [doc = " The eigenvectors of the decomposed matrix."] pub eigenvectors : OMatrix < T , D , D > , # [doc = " The unsorted eigenvalues of the decomposed matrix."] pub eigenvalues : OVector < T :: RealField , D > , }
};
}
