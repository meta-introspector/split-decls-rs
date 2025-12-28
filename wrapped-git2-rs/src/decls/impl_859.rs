macro_rules! deps {
    () => {
        Error!();
        WorktreePruneOptions!();
        Worktree!();
        Buf!();
        WorktreeLockStatus!();
        Binding!();
        Repository!();
    };
}

macro_rules! impl_859 {
    () => {
        deps!();
        impl Worktree { # [doc = " Open a worktree of a the repository"] # [doc = ""] # [doc = " If a repository is not the main tree but a worktree, this"] # [doc = " function will look up the worktree inside the parent"] # [doc = " repository and create a new `git_worktree` structure."] pub fn open_from_repository (repo : & Repository) -> Result < Worktree , Error > { let mut raw = ptr :: null_mut () ; unsafe { try_call ! (raw :: git_worktree_open_from_repository (& mut raw , repo . raw ())) ; Ok (Binding :: from_raw (raw)) } } # [doc = " Retrieves the name of the worktree"] # [doc = ""] # [doc = " This is the name that can be passed to repo::Repository::find_worktree"] # [doc = " to reopen the worktree. This is also the name that would appear in the"] # [doc = " list returned by repo::Repository::worktrees"] pub fn name (& self) -> Option < & str > { unsafe { crate :: opt_bytes (self , raw :: git_worktree_name (self . raw)) . and_then (| s | str :: from_utf8 (s) . ok ()) } } # [doc = " Retrieves the path to the worktree"] # [doc = ""] # [doc = " This is the path to the top-level of the source and not the path to the"] # [doc = " .git file within the worktree. This path can be passed to"] # [doc = " repo::Repository::open."] pub fn path (& self) -> & Path { unsafe { util :: bytes2path (crate :: opt_bytes (self , raw :: git_worktree_path (self . raw)) . unwrap ()) } } # [doc = " Validates the worktree"] # [doc = ""] # [doc = " This checks that it still exists on the"] # [doc = " filesystem and that the metadata is correct"] pub fn validate (& self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_worktree_validate (self . raw)) ; } Ok (()) } # [doc = " Locks the worktree"] pub fn lock (& self , reason : Option < & str >) -> Result < () , Error > { let reason = crate :: opt_cstr (reason) ? ; unsafe { try_call ! (raw :: git_worktree_lock (self . raw , reason)) ; } Ok (()) } # [doc = " Unlocks the worktree"] pub fn unlock (& self) -> Result < () , Error > { unsafe { try_call ! (raw :: git_worktree_unlock (self . raw)) ; } Ok (()) } # [doc = " Checks if worktree is locked"] pub fn is_locked (& self) -> Result < WorktreeLockStatus , Error > { let buf = Buf :: new () ; unsafe { match try_call ! (raw :: git_worktree_is_locked (buf . raw () , self . raw)) { 0 => Ok (WorktreeLockStatus :: Unlocked) , _ => { let v = buf . to_vec () ; Ok (WorktreeLockStatus :: Locked (match v . len () { 0 => None , _ => Some (String :: from_utf8 (v) . unwrap ()) , })) } } } } # [doc = " Prunes the worktree"] pub fn prune (& self , opts : Option < & mut WorktreePruneOptions >) -> Result < () , Error > { unsafe { try_call ! (raw :: git_worktree_prune (self . raw , opts . map (| o | o . raw ()))) ; } Ok (()) } # [doc = " Checks if the worktree is prunable"] pub fn is_prunable (& self , opts : Option < & mut WorktreePruneOptions >) -> Result < bool , Error > { unsafe { let rv = try_call ! (raw :: git_worktree_is_prunable (self . raw , opts . map (| o | o . raw ()))) ; Ok (rv != 0) } } }
    };
}

impl_859!();