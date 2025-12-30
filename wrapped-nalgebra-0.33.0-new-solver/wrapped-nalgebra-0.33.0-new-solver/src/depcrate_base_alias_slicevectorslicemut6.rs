// Generated macro for VectorSliceMut6 (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut6"}
// Dependencies: {}
# [doc = " A 6D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut6)] pub type VectorSliceMut6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorageMut < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
