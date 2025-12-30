// Generated macro for impl_712 (impl)
macro_rules! Depcrate_util_capturesimpl_712 {
() => {
// Module: crate::util::captures
// Provides: {"impl_712"}
// Dependencies: {}
impl < 'a > Iterator for CapturesPatternIter < 'a > { type Item = Option < Span > ; fn next (& mut self) -> Option < Option < Span > > { let (group_index , _) = self . names . next () ? ; Some (self . caps . get_group (group_index)) } fn size_hint (& self) -> (usize , Option < usize >) { self . names . size_hint () } fn count (self) -> usize { self . names . count () } }
};
}
