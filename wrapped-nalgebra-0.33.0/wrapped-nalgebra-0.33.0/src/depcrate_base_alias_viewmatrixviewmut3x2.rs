// Generated macro for MatrixViewMut3x2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut3x2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut3x2"}
// Dependencies: {}
# [doc = " A mutable column-major 3x2 matrix view."] # [doc = ""] # [doc = " See [`MatrixView3x2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut3x2 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U2 , ViewStorageMut < 'a , T , U3 , U2 , RStride , CStride > > ;
};
}
