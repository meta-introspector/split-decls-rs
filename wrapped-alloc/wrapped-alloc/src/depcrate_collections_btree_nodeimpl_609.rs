// Generated macro for impl_609 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_609 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_609"}
// Dependencies: {}
impl < 'a , K , V > NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > { # [doc = " # Safety"] # [doc = " Every item returned by `range` is a valid edge index for the node."] unsafe fn correct_childrens_parent_links < R : Iterator < Item = usize > > (& mut self , range : R) { for i in range { debug_assert ! (i <= self . len ()) ; unsafe { Handle :: new_edge (self . reborrow_mut () , i) } . correct_parent_link () ; } } fn correct_all_childrens_parent_links (& mut self) { let len = self . len () ; unsafe { self . correct_childrens_parent_links (0 ..= len) } ; } }
};
}
