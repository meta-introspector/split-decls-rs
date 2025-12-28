macro_rules! deps {
    () => {
        Status!();
        Kind!();
        PathspecMatch!();
        ForDeletionMode!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Status { # [doc = " Return true if this status is considered pruned. A pruned entry is typically hidden from view due to a pathspec."] pub fn is_pruned (& self) -> bool { matches ! (& self , Status :: Pruned) } # [doc = " Return `true` if `file_type` is a directory on disk and isn't ignored, and is not a repository."] # [doc = " This implements the default rules of `git status`, which is good for a minimal traversal through"] # [doc = " tracked and non-ignored portions of a worktree."] # [doc = " `for_deletion` is used to determine if recursion into a directory is allowed even though it otherwise wouldn't be."] # [doc = " If `worktree_root_is_repository` is `true`, then this status is part of the root of an iteration, and the corresponding"] # [doc = " worktree root is a repository itself. This typically happens for submodules. In this case, recursion rules are relaxed"] # [doc = " to allow traversing submodule worktrees."] # [doc = ""] # [doc = " Use `pathspec_match` to determine if a pathspec matches in any way, affecting the decision to recurse."] pub fn can_recurse (& self , file_type : Option < Kind > , pathspec_match : Option < PathspecMatch > , for_deletion : Option < ForDeletionMode > , worktree_root_is_repository : bool ,) -> bool { let is_dir_on_disk = file_type . is_some_and (| ft | { if worktree_root_is_repository { ft . is_dir () } else { ft . is_recursable_dir () } }) ; if ! is_dir_on_disk { return false ; } match self { Status :: Pruned => false , Status :: Ignored (_) => { for_deletion . is_some_and (| fd | { matches ! (fd , ForDeletionMode :: FindNonBareRepositoriesInIgnoredDirectories | ForDeletionMode :: FindRepositoriesInIgnoredDirectories) }) || pathspec_match . is_some_and (| m | ! m . should_ignore ()) } Status :: Untracked | Status :: Tracked => true , } } }
    };
}

impl_12!();