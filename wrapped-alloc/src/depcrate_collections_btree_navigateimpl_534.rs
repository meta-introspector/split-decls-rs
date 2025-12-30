// Generated macro for impl_534 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_534 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'a , K , V > LeafRange < marker :: Immut < 'a > , K , V > { # [inline] pub (super) fn next_checked (& mut self) -> Option < (& 'a K , & 'a V) > { self . perform_next_checked (| kv | kv . into_kv ()) } # [inline] pub (super) fn next_back_checked (& mut self) -> Option < (& 'a K , & 'a V) > { self . perform_next_back_checked (| kv | kv . into_kv ()) } }
};
}
