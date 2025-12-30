// Generated macro for LU (struct)
macro_rules! Depcrate_linalg_luLU {
() => {
// Module: crate::linalg::lu
// Provides: {"LU"}
// Dependencies: {}
# [doc = " LU decomposition with partial (row) pivoting."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Serialize,
         PermutationSequence<DimMinimum<R, C>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         PermutationSequence<DimMinimum<R, C>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct LU < T : ComplexField , R : DimMin < C > , C : Dim > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , { lu : OMatrix < T , R , C > , p : PermutationSequence < DimMinimum < R , C > > , }
};
}
