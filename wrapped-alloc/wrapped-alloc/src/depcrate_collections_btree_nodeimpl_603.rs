// Generated macro for impl_603 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_603 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_603"}
// Dependencies: {}
impl < K , V , Type > NodeRef < marker :: DormantMut , K , V , Type > { # [doc = " Revert to the unique borrow initially captured."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The reborrow must have ended, i.e., the reference returned by `new` and"] # [doc = " all pointers and references derived from it, must not be used anymore."] pub (super) unsafe fn awaken < 'a > (self) -> NodeRef < marker :: Mut < 'a > , K , V , Type > { NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
