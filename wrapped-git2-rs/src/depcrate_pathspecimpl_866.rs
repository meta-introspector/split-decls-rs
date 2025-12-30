// Generated macro for impl_866 (impl)
macro_rules! Depcrate_pathspecimpl_866 {
() => {
// Module: crate::pathspec
// Provides: {"impl_866"}
// Dependencies: {}
impl < 'list > DoubleEndedIterator for PathspecEntries < 'list > { fn next_back (& mut self) -> Option < & 'list [u8] > { self . range . next_back () . and_then (| i | self . list . entry (i)) } }
};
}
