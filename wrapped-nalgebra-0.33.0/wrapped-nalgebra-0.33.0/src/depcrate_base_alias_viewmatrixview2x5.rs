// Generated macro for MatrixView2x5 (type)
macro_rules! Depcrate_base_alias_viewMatrixView2x5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView2x5"}
// Dependencies: {}
# [doc = " An immutable column-major 2x5 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut2x5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView2x5 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U5 , ViewStorage < 'a , T , U2 , U5 , RStride , CStride > > ;
};
}
