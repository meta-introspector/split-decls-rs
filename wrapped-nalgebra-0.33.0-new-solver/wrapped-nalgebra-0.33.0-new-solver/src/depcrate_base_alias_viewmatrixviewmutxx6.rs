// Generated macro for MatrixViewMutXx6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMutXx6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMutXx6"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with a number of rows chosen at runtime and 6 columns."] # [doc = ""] # [doc = " See [`MatrixViewXx6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMutXx6 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U6 , ViewStorageMut < 'a , T , Dyn , U6 , RStride , CStride > > ;
};
}
