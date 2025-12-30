// Generated macro for impl_655 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_655 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_655"}
// Dependencies: {}
impl < 'a , K , V , Type > Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: LeafOrInternal > , Type > { # [doc = " Unsafely asserts to the compiler the static information that the handle's node is a `Leaf`."] pub (super) unsafe fn cast_to_leaf_unchecked (self ,) -> Handle < NodeRef < marker :: Mut < 'a > , K , V , marker :: Leaf > , Type > { let node = unsafe { self . node . cast_to_leaf_unchecked () } ; Handle { node , idx : self . idx , _marker : PhantomData } } }
};
}
