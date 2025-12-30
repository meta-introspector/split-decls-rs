// Generated macro for MatrixSliceMutXx5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx5"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 5 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx5)] pub type MatrixSliceMutXx5 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U5 , ViewStorageMut < 'a , T , Dyn , U5 , RStride , CStride > > ;
};
}
