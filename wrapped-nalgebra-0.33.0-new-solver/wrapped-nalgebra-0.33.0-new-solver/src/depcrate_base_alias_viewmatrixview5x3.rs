// Generated macro for MatrixView5x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixView5x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5x3"}
// Dependencies: {}
# [doc = " An immutable column-major 5x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut5x3`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5x3 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U3 , ViewStorage < 'a , T , U5 , U3 , RStride , CStride > > ;
};
}
