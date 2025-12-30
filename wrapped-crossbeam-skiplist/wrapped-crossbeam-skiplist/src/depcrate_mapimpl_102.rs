// Generated macro for impl_102 (impl)
macro_rules! Depcrate_mapimpl_102 {
() => {
// Module: crate::map
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a , K , V > DoubleEndedIterator for Iter < 'a , K , V > where K : Ord , { fn next_back (& mut self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . next_back (guard) . map (Entry :: new) } }
};
}
