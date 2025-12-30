// Generated macro for impl_868 (impl)
macro_rules! Depcrate_collections_linked_listimpl_868 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_868"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for LinkedList < T , A > { fn clone (& self) -> Self { let mut list = Self :: new_in (self . alloc . clone ()) ; list . extend (self . iter () . cloned ()) ; list } # [doc = " Overwrites the contents of `self` with a clone of the contents of `source`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation of the nodes of the linked list. Additionally,"] # [doc = " if the element type `T` overrides `clone_from()`, this will reuse the"] # [doc = " resources of `self`'s elements as well."] fn clone_from (& mut self , source : & Self) { let mut source_iter = source . iter () ; if self . len () > source . len () { self . split_off (source . len ()) ; } for (elem , source_elem) in self . iter_mut () . zip (& mut source_iter) { elem . clone_from (source_elem) ; } if ! source_iter . is_empty () { self . extend (source_iter . cloned ()) ; } } }
};
}
