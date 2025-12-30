// Generated macro for impl_206 (impl)
macro_rules! Depcrate_ord_mapimpl_206 {
() => {
// Module: crate::ord::map
// Provides: {"impl_206"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Keys < 'a , K , V > where K : 'a + Ord , V : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| (k , _) | k) } }
};
}
