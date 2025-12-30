// Generated macro for MatrixSlice3x6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice3x6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice3x6"}
// Dependencies: {}
# [doc = " A column-major 3x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView3x6)] pub type MatrixSlice3x6 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U6 , ViewStorage < 'a , T , U3 , U6 , RStride , CStride > > ;
};
}
