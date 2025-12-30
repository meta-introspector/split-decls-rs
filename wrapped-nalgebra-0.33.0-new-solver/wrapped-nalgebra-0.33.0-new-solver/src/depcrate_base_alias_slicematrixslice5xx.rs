// Generated macro for MatrixSlice5xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice5xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice5xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 5 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView5xX)] pub type MatrixSlice5xX < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , Dyn , ViewStorage < 'a , T , U5 , Dyn , RStride , CStride > > ;
};
}
