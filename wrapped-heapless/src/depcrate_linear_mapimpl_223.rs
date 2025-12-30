// Generated macro for impl_223 (impl)
macro_rules! Depcrate_linear_mapimpl_223 {
() => {
// Module: crate::linear_map
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (k , v) | (k as & K , v)) } }
};
}
