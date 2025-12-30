// Generated macro for impl_597 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_597 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_597"}
// Dependencies: {}
impl < BorrowType , K , V , Type > NodeRef < BorrowType , K , V , Type > { # [doc = " Finds the length of the node. This is the number of keys or values."] # [doc = " The number of edges is `len() + 1`."] # [doc = " Note that, despite being safe, calling this function can have the side effect"] # [doc = " of invalidating mutable references that unsafe code has created."] pub (super) fn len (& self) -> usize { unsafe { usize :: from ((* Self :: as_leaf_ptr (self)) . len) } } # [doc = " Returns the number of levels that the node and leaves are apart. Zero"] # [doc = " height means the node is a leaf itself. If you picture trees with the"] # [doc = " root on top, the number says at which elevation the node appears."] # [doc = " If you picture trees with leaves on top, the number says how high"] # [doc = " the tree extends above the node."] pub (super) fn height (& self) -> usize { self . height } # [doc = " Temporarily takes out another, immutable reference to the same node."] pub (super) fn reborrow (& self) -> NodeRef < marker :: Immut < '_ > , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } # [doc = " Exposes the leaf portion of any leaf or internal node."] # [doc = ""] # [doc = " Returns a raw ptr to avoid invalidating other references to this node."] fn as_leaf_ptr (this : & Self) -> * mut LeafNode < K , V > { this . node . as_ptr () } }
};
}
