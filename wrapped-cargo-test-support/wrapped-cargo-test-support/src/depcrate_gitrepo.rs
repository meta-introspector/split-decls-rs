// Generated macro for repo (function)
macro_rules! Depcrate_gitrepo {
() => {
// Module: crate::git
// Provides: {"repo"}
// Dependencies: {}
# [doc = " Create a [`RepoBuilder`] to build a new git repository."] # [doc = ""] # [doc = " Call [`RepoBuilder::build()`] to finalize and create the repository."] pub fn repo (p : & Path) -> RepoBuilder { RepoBuilder :: init (p) }
};
}
