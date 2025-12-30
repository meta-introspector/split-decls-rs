// Generated macro for impl_908 (impl)
macro_rules! Depcrate_collections_vec_deque_drainimpl_908 {
() => {
// Module: crate::collections::vec_deque::drain
// Provides: {"impl_908"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . drain_len) . field (& self . idx) . field (& self . new_len) . field (& self . remaining) . finish () } }
};
}
