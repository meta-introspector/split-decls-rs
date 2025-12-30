// Generated macro for impl_600 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_600 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_600"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , Type > NodeRef < marker :: Immut < 'a > , K , V , Type > { # [doc = " Exposes the leaf portion of any leaf or internal node in an immutable tree."] fn into_leaf (self) -> & 'a LeafNode < K , V > { let ptr = Self :: as_leaf_ptr (& self) ; unsafe { & * ptr } } # [doc = " Borrows a view into the keys stored in the node."] pub (super) fn keys (& self) -> & [K] { let leaf = self . into_leaf () ; unsafe { leaf . keys . get_unchecked (.. usize :: from (leaf . len)) . assume_init_ref () } } }
};
}
