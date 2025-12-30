// Generated macro for impl_343 (impl)
macro_rules! Depcrate_blameimpl_343 {
() => {
// Module: crate::blame
// Provides: {"impl_343"}
// Dependencies: {}
impl < 'blame > DoubleEndedIterator for BlameIter < 'blame > { fn next_back (& mut self) -> Option < BlameHunk < 'blame > > { self . range . next_back () . and_then (| i | self . blame . get_index (i)) } }
};
}
