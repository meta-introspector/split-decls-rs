// Generated macro for MatrixView1 (type)
macro_rules! Depcrate_base_alias_viewMatrixView1 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView1"}
// Dependencies: {}
# [doc = " An immutable column-major 1x1 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut1`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView1 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U1 , ViewStorage < 'a , T , U1 , U1 , RStride , CStride > > ;
};
}
