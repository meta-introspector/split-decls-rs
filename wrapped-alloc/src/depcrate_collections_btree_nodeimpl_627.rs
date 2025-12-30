// Generated macro for impl_627 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_627 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_627"}
// Dependencies: {}
impl < 'a , K , V , NodeType , HandleType > Handle < NodeRef < marker :: Mut < 'a > , K , V , NodeType > , HandleType > { # [doc = " Temporarily takes out another mutable handle on the same location. Beware, as"] # [doc = " this method is very dangerous, doubly so since it might not immediately appear"] # [doc = " dangerous."] # [doc = ""] # [doc = " For details, see `NodeRef::reborrow_mut`."] pub (super) unsafe fn reborrow_mut (& mut self ,) -> Handle < NodeRef < marker :: Mut < '_ > , K , V , NodeType > , HandleType > { Handle { node : unsafe { self . node . reborrow_mut () } , idx : self . idx , _marker : PhantomData } } # [doc = " Returns a dormant copy of this handle which can be reawakened later."] # [doc = ""] # [doc = " See `DormantMutRef` for more details."] pub (super) fn dormant (& self ,) -> Handle < NodeRef < marker :: DormantMut , K , V , NodeType > , HandleType > { Handle { node : self . node . dormant () , idx : self . idx , _marker : PhantomData } } }
};
}
