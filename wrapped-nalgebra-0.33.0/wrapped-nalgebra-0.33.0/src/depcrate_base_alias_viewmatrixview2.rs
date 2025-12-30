// Generated macro for MatrixView2 (type)
macro_rules! Depcrate_base_alias_viewMatrixView2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView2"}
// Dependencies: {}
# [doc = " An immutable column-major 2x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut2`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U2 , ViewStorage < 'a , T , U2 , U2 , RStride , CStride > > ;
};
}
