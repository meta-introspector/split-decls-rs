// Generated macro for impl_846 (impl)
macro_rules! Depcrate_collections_linked_listimpl_846 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_846"}
// Dependencies: {}
impl < 'a , T > CursorMut < 'a , T > { # [doc = " Inserts the elements from the given `LinkedList` after the current one."] # [doc = ""] # [doc = " If the cursor is pointing at the \"ghost\" non-element then the new elements are"] # [doc = " inserted at the start of the `LinkedList`."] # [unstable (feature = "linked_list_cursors" , issue = "58533")] pub fn splice_after (& mut self , list : LinkedList < T >) { unsafe { let (splice_head , splice_tail , splice_len) = match list . detach_all_nodes () { Some (parts) => parts , _ => return , } ; let node_next = match self . current { None => self . list . head , Some (node) => node . as_ref () . next , } ; self . list . splice_nodes (self . current , node_next , splice_head , splice_tail , splice_len) ; if self . current . is_none () { self . index = self . list . len ; } } } # [doc = " Inserts the elements from the given `LinkedList` before the current one."] # [doc = ""] # [doc = " If the cursor is pointing at the \"ghost\" non-element then the new elements are"] # [doc = " inserted at the end of the `LinkedList`."] # [unstable (feature = "linked_list_cursors" , issue = "58533")] pub fn splice_before (& mut self , list : LinkedList < T >) { unsafe { let (splice_head , splice_tail , splice_len) = match list . detach_all_nodes () { Some (parts) => parts , _ => return , } ; let node_prev = match self . current { None => self . list . tail , Some (node) => node . as_ref () . prev , } ; self . list . splice_nodes (node_prev , self . current , splice_head , splice_tail , splice_len) ; self . index += splice_len ; } } }
};
}
