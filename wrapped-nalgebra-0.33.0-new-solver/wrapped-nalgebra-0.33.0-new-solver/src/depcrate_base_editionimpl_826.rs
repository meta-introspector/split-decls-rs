// Generated macro for impl_826 (impl)
macro_rules! Depcrate_base_editionimpl_826 {
() => {
// Module: crate::base::edition
// Provides: {"impl_826"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , R : Dim > OMatrix < T , R , Dyn > where DefaultAllocator : Allocator < R , Dyn > , { # [doc = " Changes the number of column of this matrix in-place."] # [doc = ""] # [doc = " The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more"] # [doc = " columns than `self`, then the extra columns are filled with `val`."] # [doc = ""] # [doc = " Defined only for owned matrices with a dynamic number of columns (for example, `DVector`)."] # [cfg (any (feature = "std" , feature = "alloc"))] pub fn resize_horizontally_mut (& mut self , new_ncols : usize , val : T) where DefaultAllocator : Reallocator < T , R , Dyn , R , Dyn > , { * self = self . clone () . resize_horizontally (new_ncols , val) ; } }
};
}
