// Generated macro for DMatrixView (type)
macro_rules! Depcrate_base_alias_viewDMatrixView {
() => {
// Module: crate::base::alias_view
// Provides: {"DMatrixView"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view dynamic numbers of rows and columns."] # [doc = ""] # [doc = " See [`DMatrixViewMut`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type DMatrixView < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , Dyn , ViewStorage < 'a , T , Dyn , Dyn , RStride , CStride > > ;
};
}
