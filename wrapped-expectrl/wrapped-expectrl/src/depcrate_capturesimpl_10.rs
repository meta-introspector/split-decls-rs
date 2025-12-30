// Generated macro for impl_10 (impl)
macro_rules! Depcrate_capturesimpl_10 {
() => {
// Module: crate::captures
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a > Iterator for MatchIter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { self . matches . next () . map (| m | & self . buf [m . start () .. m . end ()]) } fn size_hint (& self) -> (usize , Option < usize >) { self . matches . size_hint () } }
};
}
