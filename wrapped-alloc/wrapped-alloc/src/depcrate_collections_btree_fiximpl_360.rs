// Generated macro for impl_360 (impl)
macro_rules! Depcrate_collections_btree_fiximpl_360 {
() => {
// Module: crate::collections::btree::fix
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , marker :: KV > { fn fix_left_border_of_left_edge < A : Allocator + Clone > (mut self , alloc : A) { while let Internal (internal_kv) = self . force () { self = internal_kv . fix_left_child (alloc . clone ()) . first_kv () ; debug_assert ! (self . reborrow () . into_node () . len () > MIN_LEN) ; } } fn fix_right_border_of_right_edge < A : Allocator + Clone > (mut self , alloc : A) { while let Internal (internal_kv) = self . force () { self = internal_kv . fix_right_child (alloc . clone ()) . last_kv () ; debug_assert ! (self . reborrow () . into_node () . len () > MIN_LEN) ; } } }
};
}
