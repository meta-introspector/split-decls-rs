// Generated macro for impl_101 (impl)
macro_rules! Depcrate_mapimpl_101 {
() => {
// Module: crate::map
// Provides: {"impl_101"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > where K : Ord , { type Item = Entry < 'a , K , V > ; fn next (& mut self) -> Option < Entry < 'a , K , V > > { let guard = & epoch :: pin () ; self . inner . next (guard) . map (Entry :: new) } }
};
}
