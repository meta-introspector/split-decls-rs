// Generated macro for impl_639 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_639 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_639"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , NodeType > Handle < NodeRef < marker :: Mut < 'a > , K , V , NodeType > , marker :: KV > { pub (super) fn key_mut (& mut self) -> & mut K { unsafe { self . node . key_area_mut (self . idx) . assume_init_mut () } } pub (super) fn into_val_mut (self) -> & 'a mut V { debug_assert ! (self . idx < self . node . len ()) ; let leaf = self . node . into_leaf_mut () ; unsafe { leaf . vals . get_unchecked_mut (self . idx) . assume_init_mut () } } pub (super) fn into_kv_mut (self) -> (& 'a mut K , & 'a mut V) { debug_assert ! (self . idx < self . node . len ()) ; let leaf = self . node . into_leaf_mut () ; let k = unsafe { leaf . keys . get_unchecked_mut (self . idx) . assume_init_mut () } ; let v = unsafe { leaf . vals . get_unchecked_mut (self . idx) . assume_init_mut () } ; (k , v) } }
};
}
