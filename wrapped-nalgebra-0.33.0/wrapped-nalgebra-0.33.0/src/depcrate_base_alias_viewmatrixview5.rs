// Generated macro for MatrixView5 (type)
macro_rules! Depcrate_base_alias_viewMatrixView5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5"}
// Dependencies: {}
# [doc = " An immutable column-major 5x5 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U5 , ViewStorage < 'a , T , U5 , U5 , RStride , CStride > > ;
};
}
