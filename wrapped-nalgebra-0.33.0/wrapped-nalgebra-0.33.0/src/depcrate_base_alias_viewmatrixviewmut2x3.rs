// Generated macro for MatrixViewMut2x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut2x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut2x3"}
// Dependencies: {}
# [doc = " A mutable column-major 2x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView2x3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut2x3 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U3 , ViewStorageMut < 'a , T , U2 , U3 , RStride , CStride > > ;
};
}
