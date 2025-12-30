// Generated macro for MatrixViewMut6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut6"}
// Dependencies: {}
# [doc = " A mutable column-major 6x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixView6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U6 , ViewStorageMut < 'a , T , U6 , U6 , RStride , CStride > > ;
};
}
