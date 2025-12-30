// Generated macro for impl_593 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_593 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_593"}
// Dependencies: {}
impl < K , V > NodeRef < marker :: Owned , K , V , marker :: Internal > { fn new_internal < A : Allocator + Clone > (child : Root < K , V > , alloc : A) -> Self { let mut new_node = unsafe { InternalNode :: new (alloc) } ; new_node . edges [0] . write (child . node) ; unsafe { NodeRef :: from_new_internal (new_node , child . height + 1) } } # [doc = " # Safety"] # [doc = " `height` must not be zero."] unsafe fn from_new_internal < A : Allocator + Clone > (internal : Box < InternalNode < K , V > , A > , height : usize ,) -> Self { debug_assert ! (height > 0) ; let node = NonNull :: from (Box :: leak (internal)) . cast () ; let mut this = NodeRef { height , node , _marker : PhantomData } ; this . borrow_mut () . correct_all_childrens_parent_links () ; this } }
};
}
