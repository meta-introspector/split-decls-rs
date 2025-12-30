// Generated macro for impl_67 (impl)
macro_rules! Depcrate_ordered_mapimpl_67 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Entries < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { self . iter . next_back () . map (| e | (& e . 0 , & e . 1)) } }
};
}
