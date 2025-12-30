// Generated macro for Outcome (struct)
macro_rules! Depcrate_index_as_worktree_with_renames_typesOutcome {
() => {
// Module: crate::index_as_worktree_with_renames::types
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Provide additional information collected during the runtime of [`index_as_worktree_with_renames()`](crate::index_as_worktree_with_renames())."] # [derive (Clone , Debug , Default)] pub struct Outcome { # [doc = " The outcome of the modification check of tracked files."] pub tracked_file_modification : crate :: index_as_worktree :: Outcome , # [doc = " The outcome of the directory walk, or `None` if its [options](Options::dirwalk) also weren't present which means"] # [doc = " the dirwalk never ran."] pub dirwalk : Option < gix_dir :: walk :: Outcome > , # [doc = " The result of the rewrite operation, if [rewrites were configured](Options::rewrites)."] pub rewrites : Option < gix_diff :: rewrites :: Outcome > , }
};
}
