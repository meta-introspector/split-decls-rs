// Generated macro for impl_632 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_632 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_632"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Inserts a new key-value pair between the key-value pairs to the right and left of"] # [doc = " this edge. This method assumes that there is enough space in the node for the new"] # [doc = " pair to fit."] unsafe fn insert_fit (mut self , key : K , val : V ,) -> Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , marker :: KV > { debug_assert ! (self . node . len () < CAPACITY) ; let new_len = self . node . len () + 1 ; unsafe { slice_insert (self . node . key_area_mut (.. new_len) , self . idx , key) ; slice_insert (self . node . val_area_mut (.. new_len) , self . idx , val) ; * self . node . len_mut () = new_len as u16 ; Handle :: new_kv (self . node , self . idx) } } }
};
}
