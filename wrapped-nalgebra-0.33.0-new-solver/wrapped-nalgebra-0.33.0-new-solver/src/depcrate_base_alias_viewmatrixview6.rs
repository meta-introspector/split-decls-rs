// Generated macro for MatrixView6 (type)
macro_rules! Depcrate_base_alias_viewMatrixView6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView6"}
// Dependencies: {}
# [doc = " An immutable column-major 6x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut6`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U6 , ViewStorage < 'a , T , U6 , U6 , RStride , CStride > > ;
};
}
