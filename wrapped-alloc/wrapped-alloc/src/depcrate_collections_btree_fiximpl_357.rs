// Generated macro for impl_357 (impl)
macro_rules! Depcrate_collections_btree_fiximpl_357 {
() => {
// Module: crate::collections::btree::fix
// Provides: {"impl_357"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Stocks up a possibly underfull node by merging with or stealing from a"] # [doc = " sibling. If successful but at the cost of shrinking the parent node,"] # [doc = " returns that shrunk parent node. Returns an `Err` if the node is"] # [doc = " an empty root."] fn fix_node_through_parent < A : Allocator + Clone > (self , alloc : A ,) -> Result < Option < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > > , Self > { let len = self . len () ; if len >= MIN_LEN { Ok (None) } else { match self . choose_parent_kv () { Ok (Left (mut left_parent_kv)) => { if left_parent_kv . can_merge () { let parent = left_parent_kv . merge_tracking_parent (alloc) ; Ok (Some (parent)) } else { left_parent_kv . bulk_steal_left (MIN_LEN - len) ; Ok (None) } } Ok (Right (mut right_parent_kv)) => { if right_parent_kv . can_merge () { let parent = right_parent_kv . merge_tracking_parent (alloc) ; Ok (Some (parent)) } else { right_parent_kv . bulk_steal_right (MIN_LEN - len) ; Ok (None) } } Err (root) => { if len > 0 { Ok (None) } else { Err (root) } } } } } }
};
}
