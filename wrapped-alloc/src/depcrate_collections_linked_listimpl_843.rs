// Generated macro for impl_843 (impl)
macro_rules! Depcrate_collections_linked_listimpl_843 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_843"}
// Dependencies: {}
# [unstable (feature = "linked_list_cursors" , issue = "58533")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for CursorMut < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("CursorMut") . field (& self . list) . field (& self . index ()) . finish () } }
};
}
