// Generated macro for impl_535 (impl)
macro_rules! Depcrate_collections_btree_navigateimpl_535 {
() => {
// Module: crate::collections::btree::navigate
// Provides: {"impl_535"}
// Dependencies: {}
impl < 'a , K , V > LeafRange < marker :: ValMut < 'a > , K , V > { # [inline] pub (super) fn next_checked (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . perform_next_checked (| kv | unsafe { ptr :: read (kv) } . into_kv_valmut ()) } # [inline] pub (super) fn next_back_checked (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . perform_next_back_checked (| kv | unsafe { ptr :: read (kv) } . into_kv_valmut ()) } }
};
}
