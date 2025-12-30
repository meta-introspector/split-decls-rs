// Generated macro for Outcome (struct)
macro_rules! Depcrate_status_iter_typesOutcome {
() => {
// Module: crate::status::iter::types
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The data the thread sends over to the receiving iterator."] pub struct Outcome { # [doc = " The outcome of the index-to-worktree comparison operation."] pub index_worktree : gix_status :: index_as_worktree_with_renames :: Outcome , # [doc = " The outcome of the diff between `HEAD^{tree}` and the index, or `None` if this outcome"] # [doc = " was produced with the [`into_index_worktree_iter()`](crate::status::Platform::into_index_worktree_iter())."] pub tree_index : Option < tree_index :: Outcome > , # [doc = " The worktree index that was used for the operation."] pub worktree_index : IndexPersistedOrInMemory , pub (super) skip_hash : bool , pub (super) changes : Option < Vec < (usize , ApplyChange) > > , }
};
}
