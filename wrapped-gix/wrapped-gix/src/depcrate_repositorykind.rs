// Generated macro for Kind (enum)
macro_rules! Depcrate_repositoryKind {
() => {
// Module: crate::repository
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " The kind of repository."] # [derive (Debug , Clone , Copy , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Kind { # [doc = " A submodule worktree, whose `git` repository lives in `.git/modules/**/<name>` of the parent repository."] # [doc = ""] # [doc = " Note that 'old-form' submodule will register as `Worktree {is_linked: false}`."] Submodule , # [doc = " A bare repository does not have a work tree, that is files on disk beyond the `git` repository itself."] Bare , # [doc = " A `git` repository along with a checked out files in a work tree."] WorkTree { # [doc = " If true, this is the git dir associated with this _linked_ worktree, otherwise it is a repository with _main_ worktree."] is_linked : bool , } , }
};
}
