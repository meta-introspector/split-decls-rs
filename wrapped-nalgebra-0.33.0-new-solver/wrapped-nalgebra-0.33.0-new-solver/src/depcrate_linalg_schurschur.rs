// Generated macro for Schur (struct)
macro_rules! Depcrate_linalg_schurSchur {
() => {
// Module: crate::linalg::schur
// Provides: {"Schur"}
// Dependencies: {}
# [doc = " Schur decomposition of a square matrix."] # [doc = ""] # [doc = " If this is a real matrix, this will be a `RealField` Schur decomposition."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<D, D>,
         OMatrix<T, D, D>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<D, D>,
         OMatrix<T, D, D>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct Schur < T : ComplexField , D : Dim > where DefaultAllocator : Allocator < D , D > , { q : OMatrix < T , D , D > , t : OMatrix < T , D , D > , }
};
}
