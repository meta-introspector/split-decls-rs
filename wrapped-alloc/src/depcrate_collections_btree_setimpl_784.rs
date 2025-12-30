// Generated macro for impl_784 (impl)
macro_rules! Depcrate_collections_btree_setimpl_784 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_784"}
// Dependencies: {}
impl < 'a , K > Cursor < 'a , K > { # [doc = " Advances the cursor to the next gap, returning the element that it"] # [doc = " moved over."] # [doc = ""] # [doc = " If the cursor is already at the end of the set then `None` is returned"] # [doc = " and the cursor is not moved."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub fn next (& mut self) -> Option < & 'a K > { self . inner . next () . map (| (k , _) | k) } # [doc = " Advances the cursor to the previous gap, returning the element that it"] # [doc = " moved over."] # [doc = ""] # [doc = " If the cursor is already at the start of the set then `None` is returned"] # [doc = " and the cursor is not moved."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub fn prev (& mut self) -> Option < & 'a K > { self . inner . prev () . map (| (k , _) | k) } # [doc = " Returns a reference to next element without moving the cursor."] # [doc = ""] # [doc = " If the cursor is at the end of the set then `None` is returned"] # [unstable (feature = "btree_cursors" , issue = "107540")] pub fn peek_next (& self) -> Option < & 'a K > { self . inner . peek_next () . map (| (k , _) | k) } # [doc = " Returns a reference to the previous element without moving the cursor."] # [doc = ""] # [doc = " If the cursor is at the start of the set then `None` is returned."] # [unstable (feature = "btree_cursors" , issue = "107540")] pub fn peek_prev (& self) -> Option < & 'a K > { self . inner . peek_prev () . map (| (k , _) | k) } }
};
}
