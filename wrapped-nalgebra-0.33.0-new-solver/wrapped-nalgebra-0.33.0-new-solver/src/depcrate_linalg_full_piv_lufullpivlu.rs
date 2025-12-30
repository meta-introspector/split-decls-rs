// Generated macro for FullPivLU (struct)
macro_rules! Depcrate_linalg_full_piv_luFullPivLU {
() => {
// Module: crate::linalg::full_piv_lu
// Provides: {"FullPivLU"}
// Dependencies: {}
# [doc = " LU decomposition with full row and column pivoting."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Serialize,
         PermutationSequence<DimMinimum<R, C>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         PermutationSequence<DimMinimum<R, C>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct FullPivLU < T : ComplexField , R : DimMin < C > , C : Dim > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , { lu : OMatrix < T , R , C > , p : PermutationSequence < DimMinimum < R , C > > , q : PermutationSequence < DimMinimum < R , C > > , }
};
}
