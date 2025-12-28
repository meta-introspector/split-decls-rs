macro_rules! deps {
    () => {
        Worktree!();
        Binding!();
    };
}

macro_rules! impl_862 {
    () => {
        deps!();
        impl Binding for Worktree { type Raw = * mut raw :: git_worktree ; unsafe fn from_raw (ptr : * mut raw :: git_worktree) -> Worktree { Worktree { raw : ptr } } fn raw (& self) -> * mut raw :: git_worktree { self . raw } }
    };
}

impl_862!();