// Generated macro for into_repo (module)
macro_rules! Depcrate_worktree_proxyinto_repo {
() => {
// Module: crate::worktree::proxy
// Provides: {"into_repo"}
// Dependencies: {}
# [allow (missing_docs)] pub mod into_repo { use std :: path :: PathBuf ; # [doc = " The error returned by [`Proxy::into_repo()`][super::Proxy::into_repo()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Open (# [from] crate :: open :: Error) , # [error ("Worktree at '{}' is inaccessible" , . base . display ())] MissingWorktree { base : PathBuf } , # [error (transparent)] MissingGitDirFile (# [from] std :: io :: Error) , } }
};
}
