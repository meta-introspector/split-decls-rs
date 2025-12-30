// Generated macro for FindMutView (struct)
macro_rules! Depcrate_sorted_linked_listFindMutView {
() => {
// Module: crate::sorted_linked_list
// Provides: {"FindMutView"}
// Dependencies: {}
# [doc = " Comes from [`SortedLinkedList::find_mut`]."] pub struct FindMutView < 'a , T , Idx , K > where T : Ord , Idx : LenType , K : Kind , { list : & 'a mut SortedLinkedListView < T , K , Idx > , is_head : bool , prev_index : Idx , index : Idx , maybe_changed : bool , }
};
}
