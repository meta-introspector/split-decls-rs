// Generated macro for impl_187 (impl)
macro_rules! Depcrate_regexset_stringimpl_187 {
() => {
// Module: crate::regexset::string
// Provides: {"impl_187"}
// Dependencies: {}
impl DoubleEndedIterator for SetMatchesIntoIter { fn next_back (& mut self) -> Option < usize > { loop { let id = self . it . next_back () ? ; if self . patset . contains (PatternID :: new_unchecked (id)) { return Some (id) ; } } } }
};
}
