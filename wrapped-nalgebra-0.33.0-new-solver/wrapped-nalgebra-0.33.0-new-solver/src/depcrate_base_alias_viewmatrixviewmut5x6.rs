// Generated macro for MatrixViewMut5x6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5x6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5x6"}
// Dependencies: {}
# [doc = " A mutable column-major 5x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixView5x6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5x6 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U6 , ViewStorageMut < 'a , T , U5 , U6 , RStride , CStride > > ;
};
}
