// Generated macro for impl_1067 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1067 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1067"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for VecDeque < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
