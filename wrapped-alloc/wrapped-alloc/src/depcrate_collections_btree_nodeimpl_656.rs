// Generated macro for impl_656 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_656 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_656"}
// Dependencies: {}
impl < 'a , K , V > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , marker :: Edge > { # [doc = " Move the suffix after `self` from one node to another one. `right` must be empty."] # [doc = " The first edge of `right` remains unchanged."] pub (super) fn move_suffix (& mut self , right : & mut NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > ,) { unsafe { let new_left_len = self . idx ; let mut left_node = self . reborrow_mut () . into_node () ; let old_left_len = left_node . len () ; let new_right_len = old_left_len - new_left_len ; let mut right_node = right . reborrow_mut () ; assert ! (right_node . len () == 0) ; assert ! (left_node . height == right_node . height) ; if new_right_len > 0 { * left_node . len_mut () = new_left_len as u16 ; * right_node . len_mut () = new_right_len as u16 ; move_to_slice (left_node . key_area_mut (new_left_len .. old_left_len) , right_node . key_area_mut (.. new_right_len) ,) ; move_to_slice (left_node . val_area_mut (new_left_len .. old_left_len) , right_node . val_area_mut (.. new_right_len) ,) ; match (left_node . force () , right_node . force ()) { (ForceResult :: Internal (mut left) , ForceResult :: Internal (mut right)) => { move_to_slice (left . edge_area_mut (new_left_len + 1 .. old_left_len + 1) , right . edge_area_mut (1 .. new_right_len + 1) ,) ; right . correct_childrens_parent_links (1 .. new_right_len + 1) ; } (ForceResult :: Leaf (_) , ForceResult :: Leaf (_)) => { } _ => unreachable ! () , } } } } }
};
}
