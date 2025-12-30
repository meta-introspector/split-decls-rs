// Generated macro for impl_640 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_640 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_640"}
// Dependencies: {}
impl < 'a , K , V , NodeType > Handle < NodeRef < marker :: ValMut < 'a > , K , V , NodeType > , marker :: KV > { pub (super) fn into_kv_valmut (self) -> (& 'a K , & 'a mut V) { unsafe { self . node . into_key_val_mut_at (self . idx) } } }
};
}
