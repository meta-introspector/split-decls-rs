macro_rules! deps {
    () => {
        Error!();
        Worktree!();
    };
}

macro_rules! into_repo {
    () => {
        deps!();
        # [allow (missing_docs)] pub mod into_repo { use std :: path :: PathBuf ; # [doc = " The error returned by [`Proxy::into_repo()`][super::Proxy::into_repo()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Open (# [from] crate :: open :: Error) , # [error ("Worktree at '{}' is inaccessible" , . base . display ())] MissingWorktree { base : PathBuf } , # [error (transparent)] MissingGitDirFile (# [from] std :: io :: Error) , } }
    };
}

into_repo!();