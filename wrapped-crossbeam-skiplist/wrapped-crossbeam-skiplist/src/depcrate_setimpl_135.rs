// Generated macro for impl_135 (impl)
macro_rules! Depcrate_setimpl_135 {
() => {
// Module: crate::set
// Provides: {"impl_135"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > where T : Ord , { type Item = Entry < 'a , T > ; fn next (& mut self) -> Option < Entry < 'a , T > > { self . inner . next () . map (Entry :: new) } }
};
}
