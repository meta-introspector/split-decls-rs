// Generated macro for MatrixSliceXx5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx5"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 5 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewXx5)] pub type MatrixSliceXx5 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U5 , ViewStorage < 'a , T , Dyn , U5 , RStride , CStride > > ;
};
}
