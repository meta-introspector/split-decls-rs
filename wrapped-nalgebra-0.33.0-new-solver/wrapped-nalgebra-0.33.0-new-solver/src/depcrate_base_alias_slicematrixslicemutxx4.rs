// Generated macro for MatrixSliceMutXx4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutXx4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutXx4"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 4 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewMutXx4)] pub type MatrixSliceMutXx4 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U4 , ViewStorageMut < 'a , T , Dyn , U4 , RStride , CStride > > ;
};
}
