// Generated macro for impl_606 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_606 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_606"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > NodeRef < marker :: Mut < 'a > , K , V , marker :: Internal > { # [doc = " Borrows exclusive access to an element or slice of the node's storage area for edge contents."] # [doc = ""] # [doc = " # Safety"] # [doc = " `index` is in bounds of 0..CAPACITY + 1"] unsafe fn edge_area_mut < I , Output : ? Sized > (& mut self , index : I) -> & mut Output where I : SliceIndex < [MaybeUninit < BoxedNode < K , V > >] , Output = Output > , { unsafe { self . as_internal_mut () . edges . as_mut_slice () . get_unchecked_mut (index) } } }
};
}
