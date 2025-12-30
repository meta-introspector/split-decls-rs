// Generated macro for impl_828 (impl)
macro_rules! Depcrate_collections_linked_listimpl_828 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_828"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for LinkedList < T , A > { fn drop (& mut self) { struct DropGuard < 'a , T , A : Allocator > (& 'a mut LinkedList < T , A >) ; impl < 'a , T , A : Allocator > Drop for DropGuard < 'a , T , A > { fn drop (& mut self) { while self . 0 . pop_front_node () . is_some () { } } } let guard = DropGuard (self) ; while guard . 0 . pop_front_node () . is_some () { } mem :: forget (guard) ; } }
};
}
