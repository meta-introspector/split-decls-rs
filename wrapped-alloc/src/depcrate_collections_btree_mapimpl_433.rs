// Generated macro for impl_433 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_433 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_433"}
// Dependencies: {}
impl < K , V , A : Allocator + Clone > IntoIter < K , V , A > { # [doc = " Core of a `next` method returning a dying KV handle,"] # [doc = " invalidated by further calls to this function and some others."] fn dying_next (& mut self ,) -> Option < Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > > { if self . length == 0 { self . range . deallocating_end (self . alloc . clone ()) ; None } else { self . length -= 1 ; Some (unsafe { self . range . deallocating_next_unchecked (self . alloc . clone ()) }) } } # [doc = " Core of a `next_back` method returning a dying KV handle,"] # [doc = " invalidated by further calls to this function and some others."] fn dying_next_back (& mut self ,) -> Option < Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > > { if self . length == 0 { self . range . deallocating_end (self . alloc . clone ()) ; None } else { self . length -= 1 ; Some (unsafe { self . range . deallocating_next_back_unchecked (self . alloc . clone ()) }) } } }
};
}
