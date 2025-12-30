// Generated macro for Outcome (struct)
macro_rules! Depcrate_dirwalkOutcome {
() => {
// Module: crate::dirwalk
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " The outcome of the [dirwalk()](crate::Repository::dirwalk)."] pub struct Outcome < 'repo > { # [doc = " The excludes stack used for the dirwalk, for access of `.gitignore` information."] pub excludes : AttributeStack < 'repo > , # [doc = " The pathspecs used to guide the operation,"] pub pathspec : Pathspec < 'repo > , # [doc = " The root actually being used for the traversal, and useful to transform the paths returned for the user."] # [doc = " It's always within the [`work-dir`](crate::Repository::workdir)."] pub traversal_root : PathBuf , # [doc = " The actual result of the dirwalk."] pub dirwalk : gix_dir :: walk :: Outcome , }
};
}
