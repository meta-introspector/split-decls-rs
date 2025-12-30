// Generated macro for impl_641 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_641 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_641"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , NodeType > Handle < NodeRef < marker :: Mut < 'a > , K , V , NodeType > , marker :: KV > { pub (super) fn kv_mut (& mut self) -> (& mut K , & mut V) { debug_assert ! (self . idx < self . node . len ()) ; unsafe { let leaf = self . node . as_leaf_mut () ; let key = leaf . keys . get_unchecked_mut (self . idx) . assume_init_mut () ; let val = leaf . vals . get_unchecked_mut (self . idx) . assume_init_mut () ; (key , val) } } # [doc = " Replaces the key and value that the KV handle refers to."] pub (super) fn replace_kv (& mut self , k : K , v : V) -> (K , V) { let (key , val) = self . kv_mut () ; (mem :: replace (key , k) , mem :: replace (val , v)) } }
};
}
