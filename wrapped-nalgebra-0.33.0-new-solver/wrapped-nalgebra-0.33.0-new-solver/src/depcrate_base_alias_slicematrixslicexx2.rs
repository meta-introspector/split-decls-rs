// Generated macro for MatrixSliceXx2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx2"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 2 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewXx2)] pub type MatrixSliceXx2 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U2 , ViewStorage < 'a , T , Dyn , U2 , RStride , CStride > > ;
};
}
