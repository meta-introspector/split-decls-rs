// Generated macro for MatrixViewMut2x6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut2x6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut2x6"}
// Dependencies: {}
# [doc = " A mutable column-major 2x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixView2x6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut2x6 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U6 , ViewStorageMut < 'a , T , U2 , U6 , RStride , CStride > > ;
};
}
