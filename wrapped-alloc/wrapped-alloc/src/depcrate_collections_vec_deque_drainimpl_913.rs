// Generated macro for impl_913 (impl)
macro_rules! Depcrate_collections_vec_deque_drainimpl_913 {
() => {
// Module: crate::collections::vec_deque::drain
// Provides: {"impl_913"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > DoubleEndedIterator for Drain < '_ , T , A > { # [inline] fn next_back (& mut self) -> Option < T > { if self . remaining == 0 { return None ; } self . remaining -= 1 ; let wrapped_idx = unsafe { self . deque . as_ref () . to_physical_idx (self . idx + self . remaining) } ; Some (unsafe { self . deque . as_mut () . buffer_read (wrapped_idx) }) } }
};
}
