// Generated macro for CursorMut (struct)
macro_rules! Depcrate_collections_linked_listCursorMut {
() => {
// Module: crate::collections::linked_list
// Provides: {"CursorMut"}
// Dependencies: {}
# [doc = " A cursor over a `LinkedList` with editing operations."] # [doc = ""] # [doc = " A `Cursor` is like an iterator, except that it can freely seek back-and-forth, and can"] # [doc = " safely mutate the list during iteration. This is because the lifetime of its yielded"] # [doc = " references is tied to its own lifetime, instead of just the underlying list. This means"] # [doc = " cursors cannot yield multiple elements at once."] # [doc = ""] # [doc = " Cursors always rest between two elements in the list, and index in a logically circular way."] # [doc = " To accommodate this, there is a \"ghost\" non-element that yields `None` between the head and"] # [doc = " tail of the list."] # [unstable (feature = "linked_list_cursors" , issue = "58533")] pub struct CursorMut < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { index : usize , current : Option < NonNull < Node < T > > > , list : & 'a mut LinkedList < T , A > , }
};
}
