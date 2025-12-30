// Generated macro for impl_47 (impl)
macro_rules! Depcrate_index_as_worktree_functionimpl_47 {
() => {
// Module: crate::index_as_worktree::function
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , T > Iterator for OffsetIter < 'a , T > { type Item = (usize , & 'a [T]) ; fn next (& mut self) -> Option < Self :: Item > { let block = self . inner . next () ? ; let offset = self . offset ; self . offset += block . len () ; Some ((offset , block)) } }
};
}
