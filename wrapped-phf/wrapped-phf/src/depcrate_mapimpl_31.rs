// Generated macro for impl_31 (impl)
macro_rules! Depcrate_mapimpl_31 {
() => {
// Module: crate::map
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Entries < 'a , K , V > { fn next_back (& mut self) -> Option < (& 'a K , & 'a V) > { self . iter . next_back () . map (| e | (& e . 0 , & e . 1)) } }
};
}
