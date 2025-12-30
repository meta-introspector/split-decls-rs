// Generated macro for impl_628 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_628 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_628"}
// Dependencies: {}
impl < K , V , NodeType , HandleType > Handle < NodeRef < marker :: DormantMut , K , V , NodeType > , HandleType > { # [doc = " Revert to the unique borrow initially captured."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The reborrow must have ended, i.e., the reference returned by `new` and"] # [doc = " all pointers and references derived from it, must not be used anymore."] pub (super) unsafe fn awaken < 'a > (self ,) -> Handle < NodeRef < marker :: Mut < 'a > , K , V , NodeType > , HandleType > { Handle { node : unsafe { self . node . awaken () } , idx : self . idx , _marker : PhantomData } } }
};
}
