// Generated macro for impl_841 (impl)
macro_rules! Depcrate_collections_linked_listimpl_841 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_841"}
// Dependencies: {}
# [unstable (feature = "linked_list_cursors" , issue = "58533")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Cursor < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Cursor") . field (& self . list) . field (& self . index ()) . finish () } }
};
}
