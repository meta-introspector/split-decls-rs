// Generated macro for impl_81 (impl)
macro_rules! Depcrate_ordered_mapimpl_81 {
() => {
// Module: crate::ordered_map
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a V > { self . iter . next_back () . map (| e | e . 1) } }
};
}
