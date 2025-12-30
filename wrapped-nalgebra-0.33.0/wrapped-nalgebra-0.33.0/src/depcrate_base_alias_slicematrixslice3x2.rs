// Generated macro for MatrixSlice3x2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice3x2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice3x2"}
// Dependencies: {}
# [doc = " A column-major 3x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView3x2)] pub type MatrixSlice3x2 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U2 , ViewStorage < 'a , T , U3 , U2 , RStride , CStride > > ;
};
}
