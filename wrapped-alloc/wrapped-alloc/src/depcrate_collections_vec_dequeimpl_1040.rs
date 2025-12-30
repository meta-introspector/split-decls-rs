// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1040 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1040"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for VecDeque < T , A > { # [track_caller] fn clone (& self) -> Self { let mut deq = Self :: with_capacity_in (self . len () , self . allocator () . clone ()) ; deq . extend (self . iter () . cloned ()) ; deq } # [doc = " Overwrites the contents of `self` with a clone of the contents of `source`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [track_caller] fn clone_from (& mut self , source : & Self) { self . clear () ; self . extend (source . iter () . cloned ()) ; } }
};
}
