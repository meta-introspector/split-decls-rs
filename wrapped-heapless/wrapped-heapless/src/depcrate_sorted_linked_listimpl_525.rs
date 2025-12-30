// Generated macro for impl_525 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_525 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_525"}
// Dependencies: {}
impl < T , Idx , K > Deref for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { type Target = T ; fn deref (& self) -> & Self :: Target { self . list . read_data_in_node_at (self . index . into_usize ()) } }
};
}
