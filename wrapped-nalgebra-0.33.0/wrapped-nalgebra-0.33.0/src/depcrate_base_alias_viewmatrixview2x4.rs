// Generated macro for MatrixView2x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixView2x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView2x4"}
// Dependencies: {}
# [doc = " An immutable column-major 2x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut2x4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView2x4 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U4 , ViewStorage < 'a , T , U2 , U4 , RStride , CStride > > ;
};
}
