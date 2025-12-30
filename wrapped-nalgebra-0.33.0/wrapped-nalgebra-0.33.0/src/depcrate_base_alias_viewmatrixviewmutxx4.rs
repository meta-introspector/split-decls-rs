// Generated macro for MatrixViewMutXx4 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMutXx4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMutXx4"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with a number of rows chosen at runtime and 4 columns."] # [doc = ""] # [doc = " See [`MatrixViewXx4`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMutXx4 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U4 , ViewStorageMut < 'a , T , Dyn , U4 , RStride , CStride > > ;
};
}
