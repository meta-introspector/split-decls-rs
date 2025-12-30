// Generated macro for impl_221 (impl)
macro_rules! Depcrate_linear_mapimpl_221 {
() => {
// Module: crate::linear_map
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , v) | (k , v)) } }
};
}
