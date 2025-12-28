macro_rules! ModulesSnapshot {
    () => {
        # [doc = " A lazily loaded and auto-updated worktree index."] pub type ModulesSnapshot = gix_fs :: SharedFileSnapshot < File > ;
    };
}

ModulesSnapshot!();