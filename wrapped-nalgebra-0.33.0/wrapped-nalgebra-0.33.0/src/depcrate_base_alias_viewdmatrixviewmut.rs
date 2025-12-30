// Generated macro for DMatrixViewMut (type)
macro_rules! Depcrate_base_alias_viewDMatrixViewMut {
() => {
// Module: crate::base::alias_view
// Provides: {"DMatrixViewMut"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view dynamic numbers of rows and columns."] # [doc = ""] # [doc = " See [`DMatrixView`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type DMatrixViewMut < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , Dyn , ViewStorageMut < 'a , T , Dyn , Dyn , RStride , CStride > > ;
};
}
