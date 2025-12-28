macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! worktree_archive {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "worktree-archive")] pub mod worktree_archive { # [doc = " The error returned by [`Repository::worktree_archive()`](crate::Repository::worktree_archive())."] pub type Error = gix_archive :: Error ; }
    };
}

worktree_archive!();