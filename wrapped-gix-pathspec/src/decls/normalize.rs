macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! normalize {
    () => {
        deps!();
        # [doc = ""] pub mod normalize { use std :: path :: PathBuf ; # [doc = " The error returned by [Pattern::normalize()](super::Pattern::normalize())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The path '{}' is not inside of the worktree '{}'" , path . display () , worktree_path . display ())] AbsolutePathOutsideOfWorktree { path : PathBuf , worktree_path : PathBuf } , # [error ("The path '{}' leaves the repository" , path . display ())] OutsideOfWorktree { path : PathBuf } , } }
    };
}

normalize!();