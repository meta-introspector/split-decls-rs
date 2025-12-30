// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1066 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1066"}
// Dependencies: {}
# [stable (feature = "extend_ref" , since = "1.2.0")] impl < 'a , T : 'a + Copy , A : Allocator > Extend < & 'a T > for VecDeque < T , A > { # [track_caller] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . spec_extend (iter . into_iter ()) ; } # [inline] # [track_caller] fn extend_one (& mut self , & elem : & 'a T) { self . push_back (elem) ; } # [inline] # [track_caller] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } # [inline] unsafe fn extend_one_unchecked (& mut self , & item : & 'a T) { unsafe { self . push_unchecked (item) ; } } }
};
}
