// Generated macro for SortedLinkedList (type)
macro_rules! Depcrate_sorted_linked_listSortedLinkedList {
() => {
// Module: crate::sorted_linked_list
// Provides: {"SortedLinkedList"}
// Dependencies: {}
# [doc = " The linked list."] pub type SortedLinkedList < T , K , const N : usize , Idx = usize > = SortedLinkedListInner < T , Idx , K , OwnedSortedLinkedListStorage < T , Idx , N > > ;
};
}
