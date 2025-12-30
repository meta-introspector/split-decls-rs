// Generated macro for impl_544 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_544 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_544"}
// Dependencies: {}
impl < 'a , K , V > LazyLeafRange < marker :: Immut < 'a > , K , V > { # [inline] pub (super) unsafe fn next_unchecked (& mut self) -> (& 'a K , & 'a V) { unsafe { self . init_front () . unwrap () . next_unchecked () } } # [inline] pub (super) unsafe fn next_back_unchecked (& mut self) -> (& 'a K , & 'a V) { unsafe { self . init_back () . unwrap () . next_back_unchecked () } } }
};
}
