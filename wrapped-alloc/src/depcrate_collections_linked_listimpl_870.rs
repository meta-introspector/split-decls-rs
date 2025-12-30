// Generated macro for impl_870 (impl)
macro_rules! Depcrate_collections_linked_listimpl_870 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_870"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Hash , A : Allocator > Hash for LinkedList < T , A > { fn hash < H : Hasher > (& self , state : & mut H) { state . write_length_prefix (self . len ()) ; for elt in self { elt . hash (state) ; } } }
};
}
