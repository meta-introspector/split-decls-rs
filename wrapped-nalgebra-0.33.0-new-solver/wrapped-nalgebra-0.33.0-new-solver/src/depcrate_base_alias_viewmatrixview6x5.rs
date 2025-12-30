// Generated macro for MatrixView6x5 (type)
macro_rules! Depcrate_base_alias_viewMatrixView6x5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView6x5"}
// Dependencies: {}
# [doc = " An immutable column-major 6x5 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut6x5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView6x5 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U5 , ViewStorage < 'a , T , U6 , U5 , RStride , CStride > > ;
};
}
