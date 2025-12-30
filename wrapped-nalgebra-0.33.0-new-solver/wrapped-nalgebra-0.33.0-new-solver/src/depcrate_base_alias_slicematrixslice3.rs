// Generated macro for MatrixSlice3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice3"}
// Dependencies: {}
# [doc = " A column-major 3x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView3)] pub type MatrixSlice3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U3 , ViewStorage < 'a , T , U3 , U3 , RStride , CStride > > ;
};
}
