// Generated macro for VectorSliceMut1 (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut1"}
// Dependencies: {}
# [doc = " A 1D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut1)] pub type VectorSliceMut1 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U1 , ViewStorageMut < 'a , T , U1 , U1 , RStride , CStride > > ;
};
}
