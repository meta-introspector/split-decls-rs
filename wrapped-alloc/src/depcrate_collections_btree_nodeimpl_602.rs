// Generated macro for impl_602 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_602 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_602"}
// Dependencies: {}
impl < 'a , K , V , Type > NodeRef < marker :: Mut < 'a > , K , V , Type > { # [doc = " Temporarily takes out another mutable reference to the same node. Beware, as"] # [doc = " this method is very dangerous, doubly so since it might not immediately appear"] # [doc = " dangerous."] # [doc = ""] # [doc = " Because mutable pointers can roam anywhere around the tree, the returned"] # [doc = " pointer can easily be used to make the original pointer dangling, out of"] # [doc = " bounds, or invalid under stacked borrow rules."] unsafe fn reborrow_mut (& mut self) -> NodeRef < marker :: Mut < '_ > , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } # [doc = " Borrows exclusive access to the leaf portion of a leaf or internal node."] fn as_leaf_mut (& mut self) -> & mut LeafNode < K , V > { let ptr = Self :: as_leaf_ptr (self) ; unsafe { & mut * ptr } } # [doc = " Offers exclusive access to the leaf portion of a leaf or internal node."] fn into_leaf_mut (mut self) -> & 'a mut LeafNode < K , V > { let ptr = Self :: as_leaf_ptr (& mut self) ; unsafe { & mut * ptr } } # [doc = " Returns a dormant copy of this node with its lifetime erased which can"] # [doc = " be reawakened later."] pub (super) fn dormant (& self) -> NodeRef < marker :: DormantMut , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
