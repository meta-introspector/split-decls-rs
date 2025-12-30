// Generated macro for MatrixViewMut6x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut6x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut6x3"}
// Dependencies: {}
# [doc = " A mutable column-major 6x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView6x3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut6x3 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U3 , ViewStorageMut < 'a , T , U6 , U3 , RStride , CStride > > ;
};
}
