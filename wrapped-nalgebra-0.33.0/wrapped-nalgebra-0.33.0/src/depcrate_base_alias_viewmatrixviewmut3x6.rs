// Generated macro for MatrixViewMut3x6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut3x6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut3x6"}
// Dependencies: {}
# [doc = " A mutable column-major 3x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixView3x6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut3x6 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U6 , ViewStorageMut < 'a , T , U3 , U6 , RStride , CStride > > ;
};
}
