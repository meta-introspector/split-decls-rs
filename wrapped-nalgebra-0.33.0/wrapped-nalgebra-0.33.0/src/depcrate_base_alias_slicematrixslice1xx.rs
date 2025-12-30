// Generated macro for MatrixSlice1xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice1xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice1xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 1 row and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixView1xX)] pub type MatrixSlice1xX < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , Dyn , ViewStorage < 'a , T , U1 , Dyn , RStride , CStride > > ;
};
}
