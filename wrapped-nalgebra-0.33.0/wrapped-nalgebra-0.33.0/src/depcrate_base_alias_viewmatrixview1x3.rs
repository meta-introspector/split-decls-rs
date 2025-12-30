// Generated macro for MatrixView1x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixView1x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView1x3"}
// Dependencies: {}
# [doc = " An immutable column-major 1x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut1x3`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView1x3 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U3 , ViewStorage < 'a , T , U1 , U3 , RStride , CStride > > ;
};
}
