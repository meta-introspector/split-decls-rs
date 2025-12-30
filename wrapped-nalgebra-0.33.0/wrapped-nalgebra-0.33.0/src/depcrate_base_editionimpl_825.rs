// Generated macro for impl_825 (impl)
macro_rules! Depcrate_base_editionimpl_825 {
() => {
// Module: crate::base::edition
// Provides: {"impl_825"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar , C : Dim > OMatrix < T , Dyn , C > where DefaultAllocator : Allocator < Dyn , C > , { # [doc = " Changes the number of rows of this matrix in-place."] # [doc = ""] # [doc = " The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more"] # [doc = " rows than `self`, then the extra rows are filled with `val`."] # [doc = ""] # [doc = " Defined only for owned matrices with a dynamic number of rows (for example, `DVector`)."] # [cfg (any (feature = "std" , feature = "alloc"))] pub fn resize_vertically_mut (& mut self , new_nrows : usize , val : T) where DefaultAllocator : Reallocator < T , Dyn , C , Dyn , C > , { * self = self . clone () . resize_vertically (new_nrows , val) ; } }
};
}
