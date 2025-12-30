// Generated macro for impl_342 (impl)
macro_rules! Depcrate_blameimpl_342 {
() => {
// Module: crate::blame
// Provides: {"impl_342"}
// Dependencies: {}
impl < 'blame > Iterator for BlameIter < 'blame > { type Item = BlameHunk < 'blame > ; fn next (& mut self) -> Option < BlameHunk < 'blame > > { self . range . next () . and_then (| i | self . blame . get_index (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
