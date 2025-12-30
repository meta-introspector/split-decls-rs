// Generated macro for MatrixViewMut4xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut4xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut4xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 4 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView4xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut4xX < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , Dyn , ViewStorageMut < 'a , T , U4 , Dyn , RStride , CStride > > ;
};
}
