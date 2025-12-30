// Generated macro for MatrixViewMutXx3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMutXx3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMutXx3"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with a number of rows chosen at runtime and 3 columns."] # [doc = ""] # [doc = " See [`MatrixViewXx3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMutXx3 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U3 , ViewStorageMut < 'a , T , Dyn , U3 , RStride , CStride > > ;
};
}
