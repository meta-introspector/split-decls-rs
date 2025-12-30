// Generated macro for impl_613 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_613 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_613"}
// Dependencies: {}
impl < K , V , Type > NodeRef < marker :: Owned , K , V , Type > { # [doc = " Mutably borrows the owned root node. Unlike `reborrow_mut`, this is safe"] # [doc = " because the return value cannot be used to destroy the root, and there"] # [doc = " cannot be other references to the tree."] pub (super) fn borrow_mut (& mut self) -> NodeRef < marker :: Mut < '_ > , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } # [doc = " Slightly mutably borrows the owned root node."] pub (super) fn borrow_valmut (& mut self) -> NodeRef < marker :: ValMut < '_ > , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } # [doc = " Irreversibly transitions to a reference that permits traversal and offers"] # [doc = " destructive methods and little else."] pub (super) fn into_dying (self) -> NodeRef < marker :: Dying , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
