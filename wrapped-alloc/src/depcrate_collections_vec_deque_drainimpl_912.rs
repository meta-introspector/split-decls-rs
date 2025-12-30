// Generated macro for impl_912 (impl)
macro_rules! Depcrate_collections_vec_deque_drainimpl_912 {
() => {
// Module: crate::collections::vec_deque::drain
// Provides: {"impl_912"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { if self . remaining == 0 { return None ; } let wrapped_idx = unsafe { self . deque . as_ref () . to_physical_idx (self . idx) } ; self . idx += 1 ; self . remaining -= 1 ; Some (unsafe { self . deque . as_mut () . buffer_read (wrapped_idx) }) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . remaining ; (len , Some (len)) } }
};
}
