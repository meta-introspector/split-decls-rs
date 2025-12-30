// Generated macro for impl_643 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_643 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_643"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , NodeType > Handle < NodeRef < marker :: Mut < 'a > , K , V , NodeType > , marker :: KV > { # [doc = " Helps implementations of `split` for a particular `NodeType`,"] # [doc = " by taking care of leaf data."] fn split_leaf_data (& mut self , new_node : & mut LeafNode < K , V >) -> (K , V) { debug_assert ! (self . idx < self . node . len ()) ; let old_len = self . node . len () ; let new_len = old_len - self . idx - 1 ; new_node . len = new_len as u16 ; unsafe { let k = self . node . key_area_mut (self . idx) . assume_init_read () ; let v = self . node . val_area_mut (self . idx) . assume_init_read () ; move_to_slice (self . node . key_area_mut (self . idx + 1 .. old_len) , & mut new_node . keys [.. new_len] ,) ; move_to_slice (self . node . val_area_mut (self . idx + 1 .. old_len) , & mut new_node . vals [.. new_len] ,) ; * self . node . len_mut () = self . idx as u16 ; (k , v) } } }
};
}
