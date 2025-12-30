// Generated macro for Context (struct)
macro_rules! Depcrate_index_as_worktree_typesContext {
() => {
// Module: crate::index_as_worktree::types
// Provides: {"Context"}
// Dependencies: {}
# [doc = " The context for [index_as_worktree()`](crate::index_as_worktree())."] # [derive (Clone)] pub struct Context < 'a > { # [doc = " The pathspec to limit the amount of paths that are checked. Can be empty to allow all paths."] # [doc = ""] # [doc = " Note that these are expected to have a [common_prefix()](gix_pathspec::Search::common_prefix()) according"] # [doc = " to the prefix of the repository to efficiently limit the scope of the paths we process."] pub pathspec : gix_pathspec :: Search , # [doc = " A stack pre-configured to allow accessing attributes for each entry, as required for `filter`"] # [doc = " and possibly pathspecs."] pub stack : gix_worktree :: Stack , # [doc = " A filter to be able to perform conversions from and to the worktree format."] # [doc = ""] # [doc = " It is needed to potentially refresh the index with data read from the worktree, which needs to be converted back"] # [doc = " to the form stored in Git."] # [doc = ""] # [doc = " Note that for this to be correct, the attribute `stack` must be configured correctly as well."] pub filter : gix_filter :: Pipeline , # [doc = " A flag to query to learn if cancellation is requested."] pub should_interrupt : & 'a AtomicBool , }
};
}
