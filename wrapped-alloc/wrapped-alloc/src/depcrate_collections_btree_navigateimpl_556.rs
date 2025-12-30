// Generated macro for impl_556 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_556 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_556"}
// Dependencies: {}
impl < 'a , K , V > Handle < NodeRef < marker :: Immut < 'a > , K , V , marker :: Leaf > , marker :: Edge > { # [doc = " Moves the leaf edge handle to the next leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a V) { super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_kv () . ok () . unwrap () ; (kv . next_leaf_edge () , kv . into_kv ()) }) } # [doc = " Moves the leaf edge handle to the previous leaf edge and returns references to the"] # [doc = " key and value in between."] # [doc = ""] # [doc = " # Safety"] # [doc = " There must be another KV in the direction travelled."] unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a V) { super :: mem :: replace (self , | leaf_edge | { let kv = leaf_edge . next_back_kv () . ok () . unwrap () ; (kv . next_back_leaf_edge () , kv . into_kv ()) }) } }
};
}
