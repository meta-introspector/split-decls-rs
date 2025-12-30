// Generated macro for MatrixViewMutXx1 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMutXx1 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMutXx1"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with a number of rows chosen at runtime and 1 column."] # [doc = ""] # [doc = " See [`MatrixViewXx1`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMutXx1 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorageMut < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
