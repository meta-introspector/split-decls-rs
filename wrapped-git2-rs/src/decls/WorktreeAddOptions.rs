macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! WorktreeAddOptions {
    () => {
        deps!();
        # [doc = " Options which can be used to configure how a worktree is initialized"] pub struct WorktreeAddOptions < 'a > { raw : raw :: git_worktree_add_options , _marker : marker :: PhantomData < Reference < 'a > > , }
    };
}

WorktreeAddOptions!();