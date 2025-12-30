// Generated macro for MatrixSlice3x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice3x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice3x1"}
// Dependencies: {}
# [doc = " A column-major 3x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView3x1)] pub type MatrixSlice3x1 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U1 , ViewStorage < 'a , T , U3 , U1 , RStride , CStride > > ;
};
}
