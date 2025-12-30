// Generated macro for impl_650 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_650 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_650"}
// Dependencies: {}
impl < T > Iterator for Values < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
