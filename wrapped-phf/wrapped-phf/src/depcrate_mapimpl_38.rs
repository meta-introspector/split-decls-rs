// Generated macro for impl_38 (impl)
macro_rules! Depcrate_mapimpl_38 {
() => {
// Module: crate::map
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > { fn next_back (& mut self) -> Option < & 'a K > { self . iter . next_back () . map (| e | e . 0) } }
};
}
