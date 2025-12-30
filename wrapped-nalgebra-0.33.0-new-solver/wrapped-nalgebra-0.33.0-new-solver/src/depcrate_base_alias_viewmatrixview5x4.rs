// Generated macro for MatrixView5x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixView5x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5x4"}
// Dependencies: {}
# [doc = " An immutable column-major 5x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut5x4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5x4 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U4 , ViewStorage < 'a , T , U5 , U4 , RStride , CStride > > ;
};
}
