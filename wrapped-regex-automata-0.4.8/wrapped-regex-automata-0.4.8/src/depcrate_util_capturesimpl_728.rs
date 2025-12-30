// Generated macro for impl_728 (impl)
macro_rules! Depcrate_util_capturesimpl_728 {
() => {
// Module: crate::util::captures
// Provides: {"impl_728"}
// Dependencies: {}
impl < 'a > Iterator for GroupInfoPatternNames < 'a > { type Item = Option < & 'a str > ; fn next (& mut self) -> Option < Option < & 'a str > > { self . it . next () . map (| x | x . as_deref ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } fn count (self) -> usize { self . it . count () } }
};
}
