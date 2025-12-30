// Generated macro for MatrixSliceMutMN (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMutMN {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMutMN"}
// Dependencies: {}
# [doc = " A column-major matrix slice with `R` rows and `C` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = "Use MatrixViewMut instead, which has an identical definition."] pub type MatrixSliceMutMN < 'a , T , R , C , RStride = U1 , CStride = R > = Matrix < T , R , C , ViewStorageMut < 'a , T , R , C , RStride , CStride > > ;
};
}
