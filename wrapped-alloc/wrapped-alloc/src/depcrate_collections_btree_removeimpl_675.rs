// Generated macro for impl_675 (impl)
macro_rules! Depcrate_collections_btree_removeimpl_675 {
() => {
// Module: crate::collections::btree::remove
// Provides: {"impl_675"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: KV > { fn remove_leaf_kv < F : FnOnce () , A : Allocator + Clone > (self , handle_emptied_internal_root : F , alloc : A ,) -> ((K , V) , Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge >) { let (old_kv , mut pos) = self . remove () ; let len = pos . reborrow () . into_node () . len () ; if len < MIN_LEN { let idx = pos . idx () ; let new_pos = match pos . into_node () . forget_type () . choose_parent_kv () { Ok (Left (left_parent_kv)) => { debug_assert ! (left_parent_kv . right_child_len () == MIN_LEN - 1) ; if left_parent_kv . can_merge () { left_parent_kv . merge_tracking_child_edge (Right (idx) , alloc . clone ()) } else { debug_assert ! (left_parent_kv . left_child_len () > MIN_LEN) ; left_parent_kv . steal_left (idx) } } Ok (Right (right_parent_kv)) => { debug_assert ! (right_parent_kv . left_child_len () == MIN_LEN - 1) ; if right_parent_kv . can_merge () { right_parent_kv . merge_tracking_child_edge (Left (idx) , alloc . clone ()) } else { debug_assert ! (right_parent_kv . right_child_len () > MIN_LEN) ; right_parent_kv . steal_right (idx) } } Err (pos) => unsafe { Handle :: new_edge (pos , idx) } , } ; pos = unsafe { new_pos . cast_to_leaf_unchecked () } ; if let Ok (parent) = unsafe { pos . reborrow_mut () } . into_node () . ascend () { if ! parent . into_node () . forget_type () . fix_node_and_affected_ancestors (alloc) { handle_emptied_internal_root () ; } } } (old_kv , pos) } }
};
}
