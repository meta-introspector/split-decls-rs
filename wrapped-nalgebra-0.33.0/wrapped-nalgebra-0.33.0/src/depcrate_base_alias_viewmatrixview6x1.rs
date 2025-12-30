// Generated macro for MatrixView6x1 (type)
macro_rules! Depcrate_base_alias_viewMatrixView6x1 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView6x1"}
// Dependencies: {}
# [doc = " An immutable column-major 6x1 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut6x1`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView6x1 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorage < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
