// Generated macro for impl_166 (impl)
macro_rules! Depcrate_regexset_bytesimpl_166 {
() => {
// Module: crate::regexset::bytes
// Provides: {"impl_166"}
// Dependencies: {}
impl Iterator for SetMatchesIntoIter { type Item = usize ; fn next (& mut self) -> Option < usize > { loop { let id = self . it . next () ? ; if self . patset . contains (PatternID :: new_unchecked (id)) { return Some (id) ; } } } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
