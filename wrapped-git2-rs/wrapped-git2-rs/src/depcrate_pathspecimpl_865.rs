// Generated macro for impl_865 (impl)
macro_rules! Depcrate_pathspecimpl_865 {
() => {
// Module: crate::pathspec
// Provides: {"impl_865"}
// Dependencies: {}
impl < 'list > Iterator for PathspecEntries < 'list > { type Item = & 'list [u8] ; fn next (& mut self) -> Option < & 'list [u8] > { self . range . next () . and_then (| i | self . list . entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
