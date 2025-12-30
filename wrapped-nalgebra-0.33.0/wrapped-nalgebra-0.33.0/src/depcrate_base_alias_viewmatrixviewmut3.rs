// Generated macro for MatrixViewMut3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut3"}
// Dependencies: {}
# [doc = " A mutable column-major 3x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U3 , ViewStorageMut < 'a , T , U3 , U3 , RStride , CStride > > ;
};
}
