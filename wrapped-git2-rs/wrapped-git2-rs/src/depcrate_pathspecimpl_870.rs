// Generated macro for impl_870 (impl)
macro_rules! Depcrate_pathspecimpl_870 {
() => {
// Module: crate::pathspec
// Provides: {"impl_870"}
// Dependencies: {}
impl < 'list > DoubleEndedIterator for PathspecDiffEntries < 'list > { fn next_back (& mut self) -> Option < DiffDelta < 'list > > { self . range . next_back () . and_then (| i | self . list . diff_entry (i)) } }
};
}
