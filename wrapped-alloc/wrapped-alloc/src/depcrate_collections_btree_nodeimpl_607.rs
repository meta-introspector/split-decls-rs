// Generated macro for impl_607 (impl)
macro_rules! Depcrate_collections_btree_nodeimpl_607 {
() => {
// Module: crate::collections::btree::node
// Provides: {"impl_607"}
// Dependencies: {}
impl < 'a , K , V , Type > NodeRef < marker :: ValMut < 'a > , K , V , Type > { # [doc = " # Safety"] # [doc = " - The node has more than `idx` initialized elements."] unsafe fn into_key_val_mut_at (mut self , idx : usize) -> (& 'a K , & 'a mut V) { let leaf = Self :: as_leaf_ptr (& mut self) ; let keys = unsafe { & raw const (* leaf) . keys } ; let vals = unsafe { & raw mut (* leaf) . vals } ; let keys : * const [_] = keys ; let vals : * mut [_] = vals ; let key = unsafe { (& * keys . get_unchecked (idx)) . assume_init_ref () } ; let val = unsafe { (& mut * vals . get_unchecked_mut (idx)) . assume_init_mut () } ; (key , val) } }
};
}
