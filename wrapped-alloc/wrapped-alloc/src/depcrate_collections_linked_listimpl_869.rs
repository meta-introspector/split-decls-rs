// Generated macro for impl_869 (impl)
macro_rules! Depcrate_collections_linked_listimpl_869 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_869"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for LinkedList < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self) . finish () } }
};
}
