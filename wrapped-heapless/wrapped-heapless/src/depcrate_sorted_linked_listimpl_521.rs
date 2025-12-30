// Generated macro for impl_521 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_521 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_521"}
// Dependencies: {}
impl < 'a , T , Idx , K > Iterator for IterView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { let index = self . index . to_non_max () ? ; let node = self . list . node_at (index) ; self . index = node . next ; Some (self . list . read_data_in_node_at (index)) } }
};
}
