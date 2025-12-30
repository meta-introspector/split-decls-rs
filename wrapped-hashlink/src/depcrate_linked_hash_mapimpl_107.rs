// Generated macro for impl_107 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_107 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for ValuesMut < 'a , K , V > { # [inline] fn next_back (& mut self) -> Option < & 'a mut V > { self . inner . next_back () . map (| e | e . 1) } }
};
}
