// Generated macro for impl_692 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_692 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_692"}
// Dependencies: {}
impl Iterator for Indices < '_ > { type Item = usize ; fn next (& mut self) -> Option < usize > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
