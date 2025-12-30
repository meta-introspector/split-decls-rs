// Generated macro for impl_655 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_655 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_655"}
// Dependencies: {}
impl < 'a , T : 'a > Iterator for ValuesRef < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
