// Generated macro for MatrixSliceMutXx2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx2"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 2 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx2)] pub type MatrixSliceMutXx2 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U2 , ViewStorageMut < 'a , T , Dyn , U2 , RStride , CStride > > ;
};
}
