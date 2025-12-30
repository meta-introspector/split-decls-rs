// Generated macro for MatrixViewMut5xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut5xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut5xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 5 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView5xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut5xX < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , Dyn , ViewStorageMut < 'a , T , U5 , Dyn , RStride , CStride > > ;
};
}
