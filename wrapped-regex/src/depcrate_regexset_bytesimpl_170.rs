// Generated macro for impl_170 (impl)
macro_rules! Depcrate_regexset_bytesimpl_170 {
() => {
// Module: crate::regexset::bytes
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a > Iterator for SetMatchesIter < 'a > { type Item = usize ; fn next (& mut self) -> Option < usize > { self . 0 . next () . map (| pid | pid . as_usize ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
