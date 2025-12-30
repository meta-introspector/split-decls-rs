// Generated macro for impl_545 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_545 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'a , K , V > LazyLeafRange < marker :: ValMut < 'a > , K , V > { # [inline] pub (super) unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a mut V) { unsafe { self . init_front () . unwrap () . next_unchecked () } } # [inline] pub (super) unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a mut V) { unsafe { self . init_back () . unwrap () . next_back_unchecked () } } }
};
}
