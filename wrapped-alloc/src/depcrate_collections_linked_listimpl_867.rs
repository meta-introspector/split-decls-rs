// Generated macro for impl_867 (impl)
macro_rules! Depcrate_collections_linked_listimpl_867 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_867"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , A : Allocator > Ord for LinkedList < T , A > { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other) } }
};
}
