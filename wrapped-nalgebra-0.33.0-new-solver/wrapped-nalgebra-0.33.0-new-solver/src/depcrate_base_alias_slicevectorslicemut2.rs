// Generated macro for VectorSliceMut2 (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut2"}
// Dependencies: {}
# [doc = " A 2D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut2)] pub type VectorSliceMut2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorageMut < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
