// Generated macro for MatrixView1xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView1xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView1xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 1 row and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut1xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView1xX < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , Dyn , ViewStorage < 'a , T , U1 , Dyn , RStride , CStride > > ;
};
}
