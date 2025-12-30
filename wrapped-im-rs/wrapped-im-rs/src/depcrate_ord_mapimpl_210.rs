// Generated macro for impl_210 (impl)
macro_rules! Depcrate_ord_mapimpl_210 {
() => {
// Module: crate::ord::map
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Values < 'a , K , V > where K : 'a + Ord , V : 'a , { fn next_back (& mut self) -> Option < Self :: Item > { self . it . next_back () . map (| (_ , v) | v) } }
};
}
