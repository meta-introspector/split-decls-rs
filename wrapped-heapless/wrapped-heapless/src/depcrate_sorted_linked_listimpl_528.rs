// Generated macro for impl_528 (impl)
macro_rules! Depcrate_sorted_linked_listimpl_528 {
() => {
// Module: crate::sorted_linked_list
// Provides: {"impl_528"}
// Dependencies: {}
impl < T , Idx , K , S > Drop for SortedLinkedListInner < T , Idx , K , S > where Idx : LenType , S : SortedLinkedListStorage < T , Idx > + ? Sized , { fn drop (& mut self) { let mut index = self . head ; while let Some (i) = index . to_non_max () { let node = self . node_at_mut (i) ; index = node . next ; unsafe { ptr :: drop_in_place (node . val . as_mut_ptr ()) ; } } } }
};
}
