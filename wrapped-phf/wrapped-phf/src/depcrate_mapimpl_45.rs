// Generated macro for impl_45 (impl)
macro_rules! Depcrate_mapimpl_45 {
() => {
// Module: crate::map
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a V > { self . iter . next_back () . map (| e | e . 1) } }
};
}
