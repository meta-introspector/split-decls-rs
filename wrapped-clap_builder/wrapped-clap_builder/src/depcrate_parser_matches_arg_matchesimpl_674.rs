// Generated macro for impl_674 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_674 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_674"}
// Dependencies: {}
impl < 'a , T > Iterator for OccurrencesRef < 'a , T > where Self : 'a , { type Item = OccurrenceValuesRef < 'a , T > ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
