// Generated macro for ColPivQR (struct)
macro_rules! Depcrate_linalg_col_piv_qrColPivQR {
() => {
// Module: crate::linalg::col_piv_qr
// Provides: {"ColPivQR"}
// Dependencies: {}
# [doc = " The QR decomposition (with column pivoting) of a general matrix."] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Serialize,
         PermutationSequence<DimMinimum<R, C>>: Serialize,
         OVector<T, DimMinimum<R, C>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "DefaultAllocator: Allocator<R, C> +
                           Allocator<DimMinimum<R, C>>,
         OMatrix<T, R, C>: Deserialize<'de>,
         PermutationSequence<DimMinimum<R, C>>: Deserialize<'de>,
         OVector<T, DimMinimum<R, C>>: Deserialize<'de>")))] # [derive (Clone , Debug)] pub struct ColPivQR < T : ComplexField , R : DimMin < C > , C : Dim > where DefaultAllocator : Allocator < R , C > + Allocator < DimMinimum < R , C > > , { col_piv_qr : OMatrix < T , R , C > , p : PermutationSequence < DimMinimum < R , C > > , diag : OVector < T , DimMinimum < R , C > > , }
};
}
