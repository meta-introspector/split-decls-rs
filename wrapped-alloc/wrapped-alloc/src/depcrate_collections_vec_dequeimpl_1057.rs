// Generated macro for impl_1057 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1057 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1057"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator > Ord for VecDeque < T , A > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
