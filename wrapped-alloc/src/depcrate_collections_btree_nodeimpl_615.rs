// Generated macro for impl_615 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_615 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_615"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > { # [doc = " Adds a key-value pair, and an edge to go to the right of that pair,"] # [doc = " to the end of the node."] pub (super) fn push (& mut self , key : K , val : V , edge : Root < K , V >) { assert ! (edge . height == self . height - 1) ; let len = self . len_mut () ; let idx = usize :: from (* len) ; assert ! (idx < CAPACITY) ; * len += 1 ; unsafe { self . key_area_mut (idx) . write (key) ; self . val_area_mut (idx) . write (val) ; self . edge_area_mut (idx + 1) . write (edge . node) ; Handle :: new_edge (self . reborrow_mut () , idx + 1) . correct_parent_link () ; } } }
};
}
