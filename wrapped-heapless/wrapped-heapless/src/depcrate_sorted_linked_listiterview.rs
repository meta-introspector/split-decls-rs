// Generated macro for IterView (struct)
macro_rules! Depcrate_sorted_linked_listIterView {
() => {
// Module: crate::sorted_linked_list
// Provides: {"IterView"}
// Dependencies: {}
# [doc = " Iterator for the linked list."] pub struct IterView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { list : & 'a SortedLinkedListInner < T , Idx , K , ViewSortedLinkedListStorage < T , Idx > > , index : Idx , }
};
}
