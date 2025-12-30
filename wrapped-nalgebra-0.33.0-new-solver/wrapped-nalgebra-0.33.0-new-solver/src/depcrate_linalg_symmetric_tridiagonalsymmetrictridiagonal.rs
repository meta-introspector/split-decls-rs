// Generated macro for SymmetricTridiagonal (struct)
macro_rules! Depcrate_linalg_symmetric_tridiagonalSymmetricTridiagonal {
() => {
// Module: crate::linalg::symmetric_tridiagonal
// Provides: {"SymmetricTridiagonal"}
// Dependencies: {}
# [doc = " Tridiagonalization of a symmetric matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Serialize,
         OVector<T, DimDiff<D, U1>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Deserialize<'de>,
         OVector<T, DimDiff<D, U1>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct SymmetricTridiagonal < T : ComplexField , D : DimSub < U1 > > where DefaultAllocator : Allocator < D , D > + Allocator < DimDiff < D , U1 > > , { tri : OMatrix < T , D , D > , off_diagonal : OVector < T , DimDiff < D , U1 > > , }
};
}
