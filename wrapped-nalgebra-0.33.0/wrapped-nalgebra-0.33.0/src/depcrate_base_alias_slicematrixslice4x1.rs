// Generated macro for MatrixSlice4x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice4x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice4x1"}
// Dependencies: {}
# [doc = " A column-major 4x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView4x1)] pub type MatrixSlice4x1 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U1 , ViewStorage < 'a , T , U4 , U1 , RStride , CStride > > ;
};
}
