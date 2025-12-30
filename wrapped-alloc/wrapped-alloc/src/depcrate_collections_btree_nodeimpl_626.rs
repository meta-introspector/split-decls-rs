// Generated macro for impl_626 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_626 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_626"}
// Dependencies: {}
impl < BorrowType , K , V , NodeType , HandleType > Handle < NodeRef < BorrowType , K , V , NodeType > , HandleType > { # [doc = " Temporarily takes out another immutable handle on the same location."] pub (super) fn reborrow (& self ,) -> Handle < NodeRef < marker :: Immut < '_ > , K , V , NodeType > , HandleType > { Handle { node : self . node . reborrow () , idx : self . idx , _marker : PhantomData } } }
};
}
