// Generated macro for MatrixViewMut1x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut1x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut1x2"}
// Dependencies: {}
# [doc = " A mutable column-major 1x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixView1x2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut1x2 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U2 , ViewStorageMut < 'a , T , U1 , U2 , RStride , CStride > > ;
};
}
