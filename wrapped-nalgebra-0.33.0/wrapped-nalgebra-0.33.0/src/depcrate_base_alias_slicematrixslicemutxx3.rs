// Generated macro for MatrixSliceMutXx3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx3"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 3 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx3)] pub type MatrixSliceMutXx3 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U3 , ViewStorageMut < 'a , T , Dyn , U3 , RStride , CStride > > ;
};
}
