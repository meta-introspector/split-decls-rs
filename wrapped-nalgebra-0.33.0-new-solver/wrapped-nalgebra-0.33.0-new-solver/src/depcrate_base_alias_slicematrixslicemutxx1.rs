// Generated macro for MatrixSliceMutXx1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx1"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 1 column."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx1)] pub type MatrixSliceMutXx1 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorageMut < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
