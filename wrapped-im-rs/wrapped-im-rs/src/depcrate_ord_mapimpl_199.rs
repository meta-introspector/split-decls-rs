// Generated macro for impl_199 (impl)
macro_rules! Depcrate_ord_mapimpl_199 {
() => {
// Module: crate::ord::map
// Provides: {"impl_199"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > where (K , V) : 'a + BTreeValue , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| (k , v) | (k , v)) } }
};
}
