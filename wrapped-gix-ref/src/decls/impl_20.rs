macro_rules! deps {
    () => {
        Category!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Category < '_ > { # [doc = " Return the prefix that would contain all references of our kind, or an empty string if the reference would"] # [doc = " be directly inside of the [`git_dir()`][crate::file::Store::git_dir()]."] pub fn prefix (& self) -> & BStr { match self { Category :: Tag => b"refs/tags/" . as_bstr () , Category :: LocalBranch => b"refs/heads/" . as_bstr () , Category :: RemoteBranch => b"refs/remotes/" . as_bstr () , Category :: Note => b"refs/notes/" . as_bstr () , Category :: MainPseudoRef => b"main-worktree/" . as_bstr () , Category :: MainRef => b"main-worktree/refs/" . as_bstr () , Category :: PseudoRef => b"" . as_bstr () , Category :: LinkedPseudoRef { .. } => b"worktrees/" . as_bstr () , Category :: LinkedRef { .. } => b"worktrees/" . as_bstr () , Category :: Bisect => b"refs/bisect/" . as_bstr () , Category :: Rewritten => b"refs/rewritten/" . as_bstr () , Category :: WorktreePrivate => b"refs/worktree/" . as_bstr () , } } # [doc = " Returns true if the category is private to their worktrees, and never shared with other worktrees."] pub fn is_worktree_private (& self) -> bool { matches ! (self , Category :: MainPseudoRef | Category :: PseudoRef | Category :: LinkedPseudoRef { .. } | Category :: WorktreePrivate | Category :: Rewritten | Category :: Bisect) } }
    };
}

impl_20!();