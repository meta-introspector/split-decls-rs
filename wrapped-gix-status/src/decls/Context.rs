macro_rules! deps {
    () => {
        DirwalkContext!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " The context for [index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames())."] pub struct Context < 'a > { # [doc = " The pathspec to limit the amount of paths that are checked. Can be empty to allow all paths."] # [doc = ""] # [doc = " Note that these are expected to have a [common_prefix()](gix_pathspec::Search::common_prefix()) according"] # [doc = " to the prefix of the repository to efficiently limit the scope of the paths we process, both for the"] # [doc = " index modifications as well as for the directory walk."] pub pathspec : gix_pathspec :: Search , # [doc = " A fully-configured platform capable of producing diffable buffers similar to what Git would do, for use"] # [doc = " with rewrite tracking."] # [doc = ""] # [doc = " Note that it contains resources that are additionally used here:"] # [doc = ""] # [doc = " * `attr_stack`"] # [doc = "     - A stack pre-configured to allow accessing attributes for each entry, as required for `filter`"] # [doc = "       and possibly pathspecs."] # [doc = "       It *may* also allow accessing `.gitignore` information for use in the directory walk."] # [doc = "       If no excludes information is present, the directory walk will identify ignored files as untracked, which"] # [doc = "       might be desirable under certain circumstances."] # [doc = " * `filter`"] # [doc = "     - A filter to be able to perform conversions from and to the worktree format."] # [doc = "       It is needed to potentially refresh the index with data read from the worktree, which needs to be converted back"] # [doc = "       to the form stored in Git."] pub resource_cache : gix_diff :: blob :: Platform , # [doc = " A flag to query to learn if cancellation is requested."] pub should_interrupt : & 'a AtomicBool , # [doc = " The context for the directory walk."] pub dirwalk : DirwalkContext < 'a > , }
    };
}

Context!()