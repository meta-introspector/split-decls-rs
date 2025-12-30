// Generated macro for MatrixViewMut6x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut6x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut6x2"}
// Dependencies: {}
# [doc = " A mutable column-major 6x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixView6x2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut6x2 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U2 , ViewStorageMut < 'a , T , U6 , U2 , RStride , CStride > > ;
};
}
