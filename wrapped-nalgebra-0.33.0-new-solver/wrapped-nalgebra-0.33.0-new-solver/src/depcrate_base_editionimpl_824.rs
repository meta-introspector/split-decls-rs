// Generated macro for impl_824 (impl)
macro_rules! Depcrate_base_editionimpl_824 {
() => {
// Module: crate::base::edition
// Provides: {"impl_824"}
// Dependencies: {}
# [doc = " # In-place resizing"] # [cfg (any (feature = "std" , feature = "alloc"))] impl < T : Scalar > OMatrix < T , Dyn , Dyn > { # [doc = " Resizes this matrix in-place."] # [doc = ""] # [doc = " The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more"] # [doc = " rows and/or columns than `self`, then the extra rows or columns are filled with `val`."] # [doc = ""] # [doc = " Defined only for owned fully-dynamic matrices, i.e., `DMatrix`."] pub fn resize_mut (& mut self , new_nrows : usize , new_ncols : usize , val : T) where DefaultAllocator : Reallocator < T , Dyn , Dyn , Dyn , Dyn > , { * self = self . clone () . resize (new_nrows , new_ncols , val) ; } }
};
}
