// Generated macro for impl_102 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_102 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a V > { self . inner . next_back () . map (| e | e . 1) } }
};
}
