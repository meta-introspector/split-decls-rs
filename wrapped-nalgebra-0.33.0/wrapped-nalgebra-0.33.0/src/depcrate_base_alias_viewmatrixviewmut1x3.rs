// Generated macro for MatrixViewMut1x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut1x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut1x3"}
// Dependencies: {}
# [doc = " A mutable column-major 1x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView1x3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut1x3 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U3 , ViewStorageMut < 'a , T , U1 , U3 , RStride , CStride > > ;
};
}
