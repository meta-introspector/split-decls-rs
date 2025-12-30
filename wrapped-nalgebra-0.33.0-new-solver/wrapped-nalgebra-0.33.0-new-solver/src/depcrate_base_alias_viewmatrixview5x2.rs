// Generated macro for MatrixView5x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixView5x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5x2"}
// Dependencies: {}
# [doc = " An immutable column-major 5x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut5x2`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5x2 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U2 , ViewStorage < 'a , T , U5 , U2 , RStride , CStride > > ;
};
}
