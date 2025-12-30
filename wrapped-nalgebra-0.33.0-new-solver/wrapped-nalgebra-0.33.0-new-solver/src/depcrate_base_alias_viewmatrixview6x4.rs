// Generated macro for MatrixView6x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixView6x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView6x4"}
// Dependencies: {}
# [doc = " An immutable column-major 6x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut6x4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView6x4 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U4 , ViewStorage < 'a , T , U6 , U4 , RStride , CStride > > ;
};
}
