// Generated macro for impl_557 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_557 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_557"}
// Dependencies: {}
impl < 'a , K , V > Handle < NodeRef < marker :: ValMut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Moves the leaf edge handle to the next leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a mut V) { let kv = super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_kv () . ok () . unwrap () ; (unsafe { ptr :: read (& kv) } . next_leaf_edge () , kv) }) ; kv . into_kv_valmut () } # [doc = " Moves the leaf edge handle to the previous leaf and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a mut V) { let kv = super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_back_kv () . ok () . unwrap () ; (unsafe { ptr :: read (& kv) } . next_back_leaf_edge () , kv) }) ; kv . into_kv_valmut () } }
};
}
