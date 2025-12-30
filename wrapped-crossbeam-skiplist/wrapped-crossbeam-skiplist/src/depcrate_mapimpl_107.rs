// Generated macro for impl_107 (impl)
macro_rules! Depcrate_mapimpl_107 {
() => {
// Module: crate::map
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a , Q , R , K , V > DoubleEndedIterator for Range < 'a , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { fn next_back (& mut self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . next_back (guard) . map (Entry :: new) } }
};
}
