// Generated macro for impl_190 (impl)
macro_rules! Depcrate_regexset_stringimpl_190 {
() => {
// Module: crate::regexset::string
// Provides: {"impl_190"}
// Dependencies: {}
impl < 'a > Iterator for SetMatchesIter < 'a > { type Item = usize ; fn next (& mut self) -> Option < usize > { self . 0 . next () . map (| pid | pid . as_usize ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
