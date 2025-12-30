// Generated macro for MatrixViewMut5 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5"}
// Dependencies: {}
# [doc = " A mutable column-major 5x5 matrix view."] # [doc = ""] # [doc = " See [`MatrixView5`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U5 , ViewStorageMut < 'a , T , U5 , U5 , RStride , CStride > > ;
};
}
