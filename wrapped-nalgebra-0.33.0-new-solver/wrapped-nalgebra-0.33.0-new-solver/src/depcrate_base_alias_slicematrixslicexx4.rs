// Generated macro for MatrixSliceXx4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceXx4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceXx4"}
// Dependencies: {}
# [doc = " A column-major matrix slice with a number of rows chosen at runtime and 4 columns."] # [deprecated = slice_deprecation_note ! (MatrixViewXx4)] pub type MatrixSliceXx4 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U4 , ViewStorage < 'a , T , Dyn , U4 , RStride , CStride > > ;
};
}
