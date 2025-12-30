// Generated macro for impl_96 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_96 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a K > { self . inner . next_back () . map (| e | e . 0) } }
};
}
