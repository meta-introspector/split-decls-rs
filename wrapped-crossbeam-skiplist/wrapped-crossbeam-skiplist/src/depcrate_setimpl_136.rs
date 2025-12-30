// Generated macro for impl_136 (impl)
macro_rules! Depcrate_setimpl_136 {
() => {
// Module: crate::set
// Provides: {"impl_136"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > where T : Ord , { fn next_back (& mut self) -> Option < Entry < 'a , T > > { self . inner . next_back () . map (Entry :: new) } }
};
}
