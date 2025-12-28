macro_rules! deps {
    () => {
        Worktree!();
    };
}

macro_rules! impl_863 {
    () => {
        deps!();
        impl Drop for Worktree { fn drop (& mut self) { unsafe { raw :: git_worktree_free (self . raw) } } }
    };
}

impl_863!()