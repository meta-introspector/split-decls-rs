// Generated macro for impl_866 (impl)
macro_rules! Depcrate_collections_linked_listimpl_866 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_866"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialOrd , A : Allocator > PartialOrd for LinkedList < T , A > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other) } }
};
}
