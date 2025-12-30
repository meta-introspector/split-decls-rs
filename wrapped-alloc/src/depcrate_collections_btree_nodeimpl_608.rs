// Generated macro for impl_608 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_608 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_608"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , Type > NodeRef < marker :: Mut < 'a > , K , V , Type > { # [doc = " Borrows exclusive access to the length of the node."] pub (super) fn len_mut (& mut self) -> & mut u16 { & mut self . as_leaf_mut () . len } }
};
}
