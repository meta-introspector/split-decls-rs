// Generated macro for impl_605 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_605 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_605"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a , Type > NodeRef < marker :: Mut < 'a > , K , V , Type > { # [doc = " Borrows exclusive access to an element of the key storage area."] # [doc = ""] # [doc = " # Safety"] # [doc = " `index` is in bounds of 0..CAPACITY"] unsafe fn key_area_mut < I , Output : ? Sized > (& mut self , index : I) -> & mut Output where I : SliceIndex < [MaybeUninit < K >] , Output = Output > , { unsafe { self . as_leaf_mut () . keys . as_mut_slice () . get_unchecked_mut (index) } } # [doc = " Borrows exclusive access to an element or slice of the node's value storage area."] # [doc = ""] # [doc = " # Safety"] # [doc = " `index` is in bounds of 0..CAPACITY"] unsafe fn val_area_mut < I , Output : ? Sized > (& mut self , index : I) -> & mut Output where I : SliceIndex < [MaybeUninit < V >] , Output = Output > , { unsafe { self . as_leaf_mut () . vals . as_mut_slice () . get_unchecked_mut (index) } } }
};
}
