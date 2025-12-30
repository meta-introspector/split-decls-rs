// Generated macro for Outcome (struct)
macro_rules! Depcrate_dirwalk_iterOutcome {
() => {
// Module: crate::dirwalk::iter
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of fully consumed [dirwalk iterator](Iter)."] pub struct Outcome { # [doc = " The index originally passed in to create the iterator."] pub index : IndexPersistedOrInMemory , # [doc = " The excludes stack used for the dirwalk, for access of `.gitignore` information."] pub excludes : gix_worktree :: Stack , # [doc = " The pathspecs used to guide the operation,"] pub pathspec : PathspecDetached , # [doc = " The root actually being used for the traversal, and useful to transform the paths returned for the user."] # [doc = " It's always within the [`work-dir`](Repository::workdir)."] pub traversal_root : PathBuf , # [doc = " The actual result of the dirwalk."] pub dirwalk : gix_dir :: walk :: Outcome , }
};
}
