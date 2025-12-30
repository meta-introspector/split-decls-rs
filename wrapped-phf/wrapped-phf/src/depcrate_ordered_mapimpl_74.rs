// Generated macro for impl_74 (impl)
macro_rules! Depcrate_ordered_mapimpl_74 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a K > { self . iter . next_back () . map (| e | e . 0) } }
};
}
