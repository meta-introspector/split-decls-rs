// Generated macro for impl_12 (impl)
macro_rules! Depcrate_index_as_worktree_typesimpl_12 {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"impl_12"}
// Dependencies: {}
impl Outcome { # [doc = " The total amount of skipped entries, i.e. those that weren't processed at all."] pub fn skipped (& self) -> usize { self . entries_skipped_by_common_prefix + self . entries_skipped_by_pathspec + self . entries_skipped_by_entry_flags } }
};
}
