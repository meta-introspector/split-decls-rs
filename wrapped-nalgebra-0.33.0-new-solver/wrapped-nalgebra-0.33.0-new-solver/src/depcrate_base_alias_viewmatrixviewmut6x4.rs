// Generated macro for MatrixViewMut6x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut6x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut6x4"}
// Dependencies: {}
# [doc = " A mutable column-major 6x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixView6x4`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut6x4 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U4 , ViewStorageMut < 'a , T , U6 , U4 , RStride , CStride > > ;
};
}
