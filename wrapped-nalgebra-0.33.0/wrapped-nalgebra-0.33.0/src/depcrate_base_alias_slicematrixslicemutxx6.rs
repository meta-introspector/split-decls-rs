// Generated macro for MatrixSliceMutXx6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx6"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 6 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx6)] pub type MatrixSliceMutXx6 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U6 , ViewStorageMut < 'a , T , Dyn , U6 , RStride , CStride > > ;
};
}
