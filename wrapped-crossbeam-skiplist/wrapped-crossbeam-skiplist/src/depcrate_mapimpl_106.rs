// Generated macro for impl_106 (impl)
macro_rules! Depcrate_mapimpl_106 {
() => {
// Module: crate::map
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a , Q , R , K , V > Iterator for Range < 'a , Q , R , K , V > where K : Ord + Comparable < Q > , R : RangeBounds < Q > , Q : ? Sized , { type Item = Entry < 'a , K , V > ; fn next (& mut self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . next (guard) . map (Entry :: new) } }
};
}
