// Generated macro for impl_619 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_619 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_619"}
// Dependencies: {}
impl < 'a , K , V > NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > { # [doc = " Unsafely asserts to the compiler the static information that this node is a `Leaf`."] pub (super) unsafe fn cast_to_leaf_unchecked (self ,) -> NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > { debug_assert ! (self . height == 0) ; NodeRef { height : self . height , node : self . node , _marker : PhantomData } } # [doc = " Unsafely asserts to the compiler the static information that this node is an `Internal`."] unsafe fn cast_to_internal_unchecked (self) -> NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > { debug_assert ! (self . height > 0) ; NodeRef { height : self . height , node : self . node , _marker : PhantomData } } }
};
}
