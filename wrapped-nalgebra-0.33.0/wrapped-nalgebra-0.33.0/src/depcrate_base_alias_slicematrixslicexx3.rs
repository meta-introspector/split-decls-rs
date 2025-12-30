// Generated macro for MatrixSliceXx3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx3"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 3 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewXx3)] pub type MatrixSliceXx3 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U3 , ViewStorage < 'a , T , Dyn , U3 , RStride , CStride > > ;
};
}
