// Generated macro for impl_674 (impl)
macro_rules! Depcrate_collections_btree_removeimpl_674 {
() => {
// Module: crate::collections::btree::remove
// Provides: {"impl_674"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , marker :: KV > { # [doc = " Removes a key-value pair from the tree, and returns that pair, as well as"] # [doc = " the leaf edge corresponding to that former pair. It's possible this empties"] # [doc = " a root node that is internal, which the caller should pop from the map"] # [doc = " holding the tree. The caller should also decrement the map's length."] pub (super) fn remove_kv_tracking < F : FnOnce () , A : Allocator + Clone > (self , handle_emptied_internal_root : F , alloc : A ,) -> ((K , V) , Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge >) { match self . force () { Leaf (node) => node . remove_leaf_kv (handle_emptied_internal_root , alloc) , Internal (node) => node . remove_internal_kv (handle_emptied_internal_root , alloc) , } } }
};
}
