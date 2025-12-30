// Generated macro for MatrixSlice3xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice3xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice3xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 3 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView3xX)] pub type MatrixSlice3xX < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , Dyn , ViewStorage < 'a , T , U3 , Dyn , RStride , CStride > > ;
};
}
