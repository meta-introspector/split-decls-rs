// Generated macro for MatrixViewMut2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut2"}
// Dependencies: {}
# [doc = " A mutable column-major 2x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixView2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U2 , ViewStorageMut < 'a , T , U2 , U2 , RStride , CStride > > ;
};
}
