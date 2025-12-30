// Generated macro for DMatrixSlice (type)
macro_rules! Depcrate_base_alias_sliceDMatrixSlice {
() => {
// Module: crate::base::alias_slice
// Provides: {"DMatrixSlice"}
// Dependencies: {}
# [doc = " A column-major matrix slice dynamic numbers of rows and columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (DMatrixView)] pub type DMatrixSlice < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , Dyn , ViewStorage < 'a , T , Dyn , Dyn , RStride , CStride > > ;
};
}
