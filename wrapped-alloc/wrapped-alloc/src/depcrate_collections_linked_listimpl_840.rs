// Generated macro for impl_840 (impl)
macro_rules! Depcrate_collections_linked_listimpl_840 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_840"}
// Dependencies: {}
# [unstable (feature = "linked_list_cursors" , issue = "58533")] impl < T , A : Allocator > Clone for Cursor < '_ , T , A > { fn clone (& self) -> Self { let Cursor { index , current , list } = * self ; Cursor { index , current , list } } }
};
}
