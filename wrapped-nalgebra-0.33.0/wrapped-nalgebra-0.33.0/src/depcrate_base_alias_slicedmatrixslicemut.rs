// Generated macro for DMatrixSliceMut (type)
macro_rules! Depcrate_base_alias_sliceDMatrixSliceMut {
() => {
// Module: crate::base::alias_slice
// Provides: {"DMatrixSliceMut"}
// Dependencies: {}
# [doc = " A column-major matrix slice dynamic numbers of rows and columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (DMatrixViewMut)] pub type DMatrixSliceMut < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , Dyn , ViewStorageMut < 'a , T , Dyn , Dyn , RStride , CStride > > ;
};
}
