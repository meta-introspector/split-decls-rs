// Generated macro for impl_642 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_642 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_642"}
// Dependencies: {}
impl < K , V , NodeType > Handle < NodeRef < marker :: Dying , K , V , NodeType > , marker :: KV > { # [doc = " Extracts the key and value that the KV handle refers to."] # [doc = " # Safety"] # [doc = " The node that the handle refers to must not yet have been deallocated."] pub (super) unsafe fn into_key_val (mut self) -> (K , V) { debug_assert ! (self . idx < self . node . len ()) ; let leaf = self . node . as_leaf_dying () ; unsafe { let key = leaf . keys . get_unchecked_mut (self . idx) . assume_init_read () ; let val = leaf . vals . get_unchecked_mut (self . idx) . assume_init_read () ; (key , val) } } # [doc = " Drops the key and value that the KV handle refers to."] # [doc = " # Safety"] # [doc = " The node that the handle refers to must not yet have been deallocated."] # [inline] pub (super) unsafe fn drop_key_val (mut self) { struct Dropper < 'a , T > (& 'a mut MaybeUninit < T >) ; impl < T > Drop for Dropper < '_ , T > { # [inline] fn drop (& mut self) { unsafe { self . 0 . assume_init_drop () ; } } } debug_assert ! (self . idx < self . node . len ()) ; let leaf = self . node . as_leaf_dying () ; unsafe { let key = leaf . keys . get_unchecked_mut (self . idx) ; let val = leaf . vals . get_unchecked_mut (self . idx) ; let _guard = Dropper (val) ; key . assume_init_drop () ; } } }
};
}
