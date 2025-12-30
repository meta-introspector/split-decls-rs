// Generated macro for impl_656 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_656 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_656"}
// Dependencies: {}
impl < 'a , T : 'a > DoubleEndedIterator for ValuesRef < 'a , T > { fn next_back (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next_back () { self . len -= 1 ; Some (next) } else { None } } }
};
}
