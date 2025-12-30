// Generated macro for MatrixSlice2xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice2xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice2xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 2 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView2xX)] pub type MatrixSlice2xX < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , Dyn , ViewStorage < 'a , T , U2 , Dyn , RStride , CStride > > ;
};
}
