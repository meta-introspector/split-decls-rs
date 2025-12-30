// Generated macro for MatrixViewMut5x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5x3"}
// Dependencies: {}
# [doc = " A mutable column-major 5x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView5x3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5x3 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U3 , ViewStorageMut < 'a , T , U5 , U3 , RStride , CStride > > ;
};
}
