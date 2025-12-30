// Generated macro for impl_546 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_546 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_546"}
// Dependencies: {}
impl < K , V > LazyLeafRange < marker :: Dying , K , V > { fn take_front (& mut self ,) -> Option < Handle < NodeRef < marker :: Dying , K , V , marker :: Leaf > , marker :: Edge > > { match self . front . take () ? { LazyLeafHandle :: Root (root) => Some (root . first_leaf_edge ()) , LazyLeafHandle :: Edge (edge) => Some (edge) , } } # [inline] pub (super) unsafe fn deallocating_next_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { debug_assert ! (self . front . is_some ()) ; let front = self . init_front () . unwrap () ; unsafe { front . deallocating_next_unchecked (alloc) } } # [inline] pub (super) unsafe fn deallocating_next_back_unchecked < A : Allocator + Clone > (& mut self , alloc : A ,) -> Handle < NodeRef < marker :: Dying , K , V , marker :: LeafOrInternal > , marker :: KV > { debug_assert ! (self . back . is_some ()) ; let back = self . init_back () . unwrap () ; unsafe { back . deallocating_next_back_unchecked (alloc) } } # [inline] pub (super) fn deallocating_end < A : Allocator + Clone > (& mut self , alloc : A) { if let Some (front) = self . take_front () { front . deallocating_end (alloc) } } }
};
}
