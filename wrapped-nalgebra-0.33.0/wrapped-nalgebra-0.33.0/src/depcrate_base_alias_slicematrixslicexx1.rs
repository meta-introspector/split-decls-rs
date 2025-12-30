// Generated macro for MatrixSliceXx1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx1"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 1 column."] # [deprecated = slice_deprecation_note ! (MatrixViewXx1)] pub type MatrixSliceXx1 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorage < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
