// Generated macro for impl_561 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_561 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_561"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Immut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Visits leaf nodes and internal KVs in order of ascending keys, and also"] # [doc = " visits internal nodes as a whole in a depth first order, meaning that"] # [doc = " internal nodes precede their individual KVs and their child nodes."] pub (super) fn visit_nodes_in_order < F > (self , mut visit : F) where F : FnMut (Position < marker :: Immut < 'a > , K , V >) , { match self . force () { Leaf (leaf) => visit (Position :: Leaf (leaf)) , Internal (internal) => { visit (Position :: Internal (internal)) ; let mut edge = internal . first_edge () ; loop { edge = match edge . descend () . force () { Leaf (leaf) => { visit (Position :: Leaf (leaf)) ; match edge . next_kv () { Ok (kv) => { visit (Position :: InternalKV) ; kv . right_edge () } Err (_) => return , } } Internal (internal) => { visit (Position :: Internal (internal)) ; internal . first_edge () } } } } } } # [doc = " Calculates the number of elements in a (sub)tree."] pub (super) fn calc_length (self) -> usize { let mut result = 0 ; self . visit_nodes_in_order (| pos | match pos { Position :: Leaf (node) => result += node . len () , Position :: Internal (node) => result += node . len () , Position :: InternalKV => () , }) ; result } }
};
}
