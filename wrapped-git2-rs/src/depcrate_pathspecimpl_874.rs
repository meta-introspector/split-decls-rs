// Generated macro for impl_874 (impl)
macro_rules! Depcrate_pathspecimpl_874 {
() => {
// Module: crate::pathspec
// Provides: {"impl_874"}
// Dependencies: {}
impl < 'list > DoubleEndedIterator for PathspecFailedEntries < 'list > { fn next_back (& mut self) -> Option < & 'list [u8] > { self . range . next_back () . and_then (| i | self . list . failed_entry (i)) } }
};
}
