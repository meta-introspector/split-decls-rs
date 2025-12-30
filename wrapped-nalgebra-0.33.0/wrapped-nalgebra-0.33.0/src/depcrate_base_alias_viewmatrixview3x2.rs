// Generated macro for MatrixView3x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixView3x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView3x2"}
// Dependencies: {}
# [doc = " An immutable column-major 3x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut3x2`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView3x2 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U2 , ViewStorage < 'a , T , U3 , U2 , RStride , CStride > > ;
};
}
