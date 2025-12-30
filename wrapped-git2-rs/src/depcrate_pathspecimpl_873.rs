// Generated macro for impl_873 (impl)
macro_rules! Depcrate_pathspecimpl_873 {
() => {
// Module: crate::pathspec
// Provides: {"impl_873"}
// Dependencies: {}
impl < 'list > Iterator for PathspecFailedEntries < 'list > { type Item = & 'list [u8] ; fn next (& mut self) -> Option < & 'list [u8] > { self . range . next () . and_then (| i | self . list . failed_entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
