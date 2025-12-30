// Generated macro for impl_980 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_extendimpl_980 {
() => {
// Module: crate::collections::vec_deque::spec_extend
// Provides: {"impl_980"}
// Dependencies: {}
impl < 'a , T : 'a , A : Allocator > SpecExtend < & 'a T , slice :: Iter < 'a , T > > for VecDeque < T , A > where T : Copy , { # [track_caller] fn spec_extend (& mut self , iterator : slice :: Iter < 'a , T >) { let slice = iterator . as_slice () ; self . reserve (slice . len ()) ; unsafe { self . copy_slice (self . to_physical_idx (self . len) , slice) ; self . len += slice . len () ; } } }
};
}
