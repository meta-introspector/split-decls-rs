// Generated macro for impl_638 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_638 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_638"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , NodeType > Handle < NodeRef < marker :: Immut < 'a > , K , V , NodeType > , marker :: KV > { pub (super) fn into_kv (self) -> (& 'a K , & 'a V) { debug_assert ! (self . idx < self . node . len ()) ; let leaf = self . node . into_leaf () ; let k = unsafe { leaf . keys . get_unchecked (self . idx) . assume_init_ref () } ; let v = unsafe { leaf . vals . get_unchecked (self . idx) . assume_init_ref () } ; (k , v) } }
};
}
