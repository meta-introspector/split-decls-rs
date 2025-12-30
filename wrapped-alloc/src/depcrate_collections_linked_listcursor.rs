// Generated macro for Cursor (struct)
macro_rules! Depcrate_collections_linked_listCursor {
() => {
// Module: crate::collections::linked_list
// Provides: {"Cursor"}
// Dependencies: {}
# [doc = " A cursor over a `LinkedList`."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth."] # [doc = ""] # [doc = " Cursors always rest between two elements in the list, and index in a logically circular way."] # [doc = " To accommodate this, there is a \"ghost\" non-element that yields `None` between the head and"] # [doc = " tail of the list."] # [doc = ""] # [doc = " When created, cursors start at the front of the list, or the \"ghost\" non-element if the list is empty."] # [unstable (feature = "linked_list_cursors" , issue = "58533")] pub struct Cursor < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { index : usize , current : Option < NonNull < Node < T > > > , list : & 'a LinkedList < T , A > , }
};
}
