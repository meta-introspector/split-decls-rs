// Generated macro for Hessenberg (struct)
macro_rules! Depcrate_linalg_hessenbergHessenberg {
() => {
// Module: crate::linalg::hessenberg
// Provides: {"Hessenberg"}
// Dependencies: {}
# [doc = " Hessenberg decomposition of a general matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Serialize,
         OVector<T, DimDiff<D, U1>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<DimDiff<D, U1>>,
         OMatrix<T, D, D>: Deserialize<'de>,
         OVector<T, DimDiff<D, U1>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct Hessenberg < T : ComplexField , D : DimSub < U1 > > where DefaultAllocator : Allocator < D , D > + Allocator < DimDiff < D , U1 > > , { hess : OMatrix < T , D , D > , subdiag : OVector < T , DimDiff < D , U1 > > , }
};
}
