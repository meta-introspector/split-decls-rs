macro_rules! deps {
    () => {
        WorktreePruneOptions!();
    };
}

macro_rules! impl_861 {
    () => {
        deps!();
        impl WorktreePruneOptions { # [doc = " Creates a default set of pruning options"] # [doc = ""] # [doc = " By defaults this will prune only worktrees that are no longer valid"] # [doc = " unlocked and not checked out"] pub fn new () -> WorktreePruneOptions { unsafe { let mut raw = mem :: zeroed () ; assert_eq ! (raw :: git_worktree_prune_options_init (& mut raw , raw :: GIT_WORKTREE_PRUNE_OPTIONS_VERSION) , 0) ; WorktreePruneOptions { raw } } } # [doc = " Controls whether valid (still existing on the filesystem) worktrees"] # [doc = " will be pruned"] # [doc = ""] # [doc = " Defaults to false"] pub fn valid (& mut self , valid : bool) -> & mut WorktreePruneOptions { self . flag (raw :: GIT_WORKTREE_PRUNE_VALID , valid) } # [doc = " Controls whether locked worktrees will be pruned"] # [doc = ""] # [doc = " Defaults to false"] pub fn locked (& mut self , locked : bool) -> & mut WorktreePruneOptions { self . flag (raw :: GIT_WORKTREE_PRUNE_LOCKED , locked) } # [doc = " Controls whether the actual working tree on the filesystem is recursively removed"] # [doc = ""] # [doc = " Defaults to false"] pub fn working_tree (& mut self , working_tree : bool) -> & mut WorktreePruneOptions { self . flag (raw :: GIT_WORKTREE_PRUNE_WORKING_TREE , working_tree) } fn flag (& mut self , flag : raw :: git_worktree_prune_t , on : bool) -> & mut WorktreePruneOptions { if on { self . raw . flags |= flag as u32 ; } else { self . raw . flags &= ! (flag as u32) ; } self } # [doc = " Get a set of raw prune options to be used with `git_worktree_prune`"] pub fn raw (& mut self) -> * mut raw :: git_worktree_prune_options { & mut self . raw } }
    };
}

impl_861!();