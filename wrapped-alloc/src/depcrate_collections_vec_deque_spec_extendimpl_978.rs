// Generated macro for impl_978 (impl)
macro_rules! Depcrate_collections_vec_deque_spec_extendimpl_978 {
() => {
// Module: crate::collections::vec_deque::spec_extend
// Provides: {"impl_978"}
// Dependencies: {}
# [cfg (not (test))] impl < T , A : Allocator > SpecExtend < T , vec :: IntoIter < T > > for VecDeque < T , A > { # [track_caller] fn spec_extend (& mut self , mut iterator : vec :: IntoIter < T >) { let slice = iterator . as_slice () ; self . reserve (slice . len ()) ; unsafe { self . copy_slice (self . to_physical_idx (self . len) , slice) ; self . len += slice . len () ; } iterator . forget_remaining_elements () ; } }
};
}
