// Generated macro for impl_822 (impl)
macro_rules! Depcrate_collections_linked_listimpl_822 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_822"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . list) . finish () } }
};
}
