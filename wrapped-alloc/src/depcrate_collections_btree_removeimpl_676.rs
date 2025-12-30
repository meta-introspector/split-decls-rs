// Generated macro for impl_676 (impl)
macro_rules! Depcrate_collections_btree_removeimpl_676 {
() => {
// Module: crate::collections::btree::remove
// Provides: {"impl_676"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > , marker :: KV > { fn remove_internal_kv < F : FnOnce () , A : Allocator + Clone > (self , handle_emptied_internal_root : F , alloc : A ,) -> ((K , V) , Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge >) { let left_leaf_kv = self . left_edge () . descend () . last_leaf_edge () . left_kv () ; let left_leaf_kv = unsafe { left_leaf_kv . ok () . unwrap_unchecked () } ; let (left_kv , left_hole) = left_leaf_kv . remove_leaf_kv (handle_emptied_internal_root , alloc) ; let mut internal = unsafe { left_hole . next_kv () . ok () . unwrap_unchecked () } ; let old_kv = internal . replace_kv (left_kv . 0 , left_kv . 1) ; let pos = internal . next_leaf_edge () ; (old_kv , pos) } }
};
}
