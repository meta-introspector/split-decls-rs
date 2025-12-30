// Generated macro for impl_167 (impl)
macro_rules! Depcrate_regexset_bytesimpl_167 {
() => {
// Module: crate::regexset::bytes
// Provides: {"impl_167"}
// Dependencies: {}
impl DoubleEndedIterator for SetMatchesIntoIter { fn next_back (& mut self) -> Option < usize > { loop { let id = self . it . next_back () ? ; if self . patset . contains (PatternID :: new_unchecked (id)) { return Some (id) ; } } } }
};
}
