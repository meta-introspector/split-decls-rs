// Generated macro for impl_660 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_660 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_660"}
// Dependencies: {}
impl < 'a > Iterator for RawValues < 'a > { type Item = & 'a OsStr ; fn next (& mut self) -> Option < & 'a OsStr > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
};
}
