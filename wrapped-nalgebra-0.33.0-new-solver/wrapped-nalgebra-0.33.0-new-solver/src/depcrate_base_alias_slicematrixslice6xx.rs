// Generated macro for MatrixSlice6xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice6xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice6xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 6 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView6xX)] pub type MatrixSlice6xX < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , Dyn , ViewStorage < 'a , T , U6 , Dyn , RStride , CStride > > ;
};
}
