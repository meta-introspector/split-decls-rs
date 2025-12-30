// Generated macro for MatrixViewMut5x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5x4"}
// Dependencies: {}
# [doc = " A mutable column-major 5x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixView5x4`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5x4 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U4 , ViewStorageMut < 'a , T , U5 , U4 , RStride , CStride > > ;
};
}
