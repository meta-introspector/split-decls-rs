// Generated macro for MatrixViewMut1xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut1xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut1xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 1 row and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView1xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut1xX < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , Dyn , ViewStorageMut < 'a , T , U1 , Dyn , RStride , CStride > > ;
};
}
