// Generated macro for MatrixViewMut3xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut3xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut3xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 3 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView3xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut3xX < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , Dyn , ViewStorageMut < 'a , T , U3 , Dyn , RStride , CStride > > ;
};
}
