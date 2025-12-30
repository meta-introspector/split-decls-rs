// Generated macro for impl_526 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_526 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_526"}
// Dependencies: {}
impl < T , Idx , K > DerefMut for FindMutView < '_ , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { fn deref_mut (& mut self) -> & mut Self :: Target { self . maybe_changed = true ; self . list . read_mut_data_in_node_at (self . index . into_usize ()) } }
};
}
