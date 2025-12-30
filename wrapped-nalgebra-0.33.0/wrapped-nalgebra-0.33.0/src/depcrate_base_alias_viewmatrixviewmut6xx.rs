// Generated macro for MatrixViewMut6xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut6xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut6xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 6 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView6xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut6xX < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , Dyn , ViewStorageMut < 'a , T , U6 , Dyn , RStride , CStride > > ;
};
}
