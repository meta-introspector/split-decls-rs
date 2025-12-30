// Generated macro for impl_1056 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1056 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1056"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialOrd , A : Allocator > PartialOrd for VecDeque < T , A > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
