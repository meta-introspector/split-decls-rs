// Generated macro for MatrixViewMut5x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5x2"}
// Dependencies: {}
# [doc = " A mutable column-major 5x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixView5x2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5x2 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U2 , ViewStorageMut < 'a , T , U5 , U2 , RStride , CStride > > ;
};
}
