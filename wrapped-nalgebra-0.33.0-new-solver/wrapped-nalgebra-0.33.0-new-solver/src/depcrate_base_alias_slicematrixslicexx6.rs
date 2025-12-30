// Generated macro for MatrixSliceXx6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx6"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 6 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewXx6)] pub type MatrixSliceXx6 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U6 , ViewStorage < 'a , T , Dyn , U6 , RStride , CStride > > ;
};
}
