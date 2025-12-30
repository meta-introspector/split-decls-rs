// Generated macro for impl_869 (impl)
macro_rules! Depcrate_pathspecimpl_869 {
() => {
// Module: crate::pathspec
// Provides: {"impl_869"}
// Dependencies: {}
impl < 'list > Iterator for PathspecDiffEntries < 'list > { type Item = DiffDelta < 'list > ; fn next (& mut self) -> Option < DiffDelta < 'list > > { self . range . next () . and_then (| i | self . list . diff_entry (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
};
}
