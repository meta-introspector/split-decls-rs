// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1058 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1058"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Hash , A : Allocator > Hash for VecDeque < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_length_prefix (self . len) ; self . iter () . for_each (| elem | elem . hash (state)) ; } }
};
}
