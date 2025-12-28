macro_rules! deps {
    () => {
        Proxy!();
        Worktree!();
        Note!();
        Path!();
        Read!();
    };
}

macro_rules! impl_845 {
    () => {
        deps!();
        # [doc = " Access"] impl < 'repo > crate :: Worktree < 'repo > { # [doc = " Read the location of the checkout, the base of the work tree"] pub fn base (& self) -> & 'repo std :: path :: Path { self . path } # [doc = " Return true if this worktree is the main worktree associated with a non-bare git repository."] # [doc = ""] # [doc = " It cannot be removed."] pub fn is_main (& self) -> bool { self . id () . is_none () } # [doc = " Return true if this worktree cannot be pruned, moved or deleted, which is useful if it is located on an external storage device."] # [doc = ""] # [doc = " Always false for the main worktree."] pub fn is_locked (& self) -> bool { Proxy :: new (self . parent , self . parent . git_dir ()) . is_locked () } # [doc = " Provide a reason for the locking of this worktree, if it is locked at all."] # [doc = ""] # [doc = " Note that we squelch errors in case the file cannot be read in which case the"] # [doc = " reason is an empty string."] pub fn lock_reason (& self) -> Option < BString > { Proxy :: new (self . parent , self . parent . git_dir ()) . lock_reason () } # [doc = " Return the ID of the repository worktree, if it is a linked worktree, or `None` if it's a linked worktree."] pub fn id (& self) -> Option < & BStr > { id (self . parent . git_dir () , self . parent . common_dir . is_some ()) } # [doc = " Returns true if the `.git` file or directory exists within the worktree."] # [doc = ""] # [doc = " This is an indicator for the worktree to be checked out particularly if the parent repository is a submodule."] pub fn dot_git_exists (& self) -> bool { self . path . join (gix_discover :: DOT_GIT_DIR) . exists () } }
    };
}

impl_845!()