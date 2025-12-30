// Generated macro for MatrixView5x6 (type)
macro_rules! Depcrate_base_alias_viewMatrixView5x6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5x6"}
// Dependencies: {}
# [doc = " An immutable column-major 5x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut5x6`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5x6 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U6 , ViewStorage < 'a , T , U5 , U6 , RStride , CStride > > ;
};
}
