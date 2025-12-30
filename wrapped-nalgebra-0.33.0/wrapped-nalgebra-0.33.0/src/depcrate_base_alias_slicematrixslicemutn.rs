// Generated macro for MatrixSliceMutN (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutN {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutN"}
// Dependencies: {}
# [doc = " A column-major matrix slice with `D` rows and columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = "Use MatrixViewMut instead."] pub type MatrixSliceMutN < 'a , T , D , RStride = U1 , CStride = D > = Matrix < T , D , D , ViewStorageMut < 'a , T , D , D , RStride , CStride > > ;
};
}
