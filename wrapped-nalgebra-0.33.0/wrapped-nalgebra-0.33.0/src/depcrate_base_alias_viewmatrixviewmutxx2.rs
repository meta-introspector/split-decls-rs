// Generated macro for MatrixViewMutXx2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMutXx2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMutXx2"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with a number of rows chosen at runtime and 2 columns."] # [doc = ""] # [doc = " See [`MatrixViewXx2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMutXx2 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U2 , ViewStorageMut < 'a , T , Dyn , U2 , RStride , CStride > > ;
};
}
