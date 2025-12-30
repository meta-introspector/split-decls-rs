// Generated macro for MatrixSlice4xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice4xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice4xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 4 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView4xX)] pub type MatrixSlice4xX < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , Dyn , ViewStorage < 'a , T , U4 , Dyn , RStride , CStride > > ;
};
}
