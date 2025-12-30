// Generated macro for impl_1065 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1065 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1065"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Extend < T > for VecDeque < T , A > { # [track_caller] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { < Self as SpecExtend < T , I :: IntoIter > > :: spec_extend (self , iter . into_iter ()) ; } # [inline] # [track_caller] fn extend_one (& mut self , elem : T) { self . push_back (elem) ; } # [inline] # [track_caller] fn extend_reserve (& mut self , additional : usize) { self . reserve (additional) ; } # [inline] unsafe fn extend_one_unchecked (& mut self , item : T) { unsafe { self . push_unchecked (item) ; } } }
};
}
