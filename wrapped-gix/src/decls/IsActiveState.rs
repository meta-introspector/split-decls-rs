macro_rules! IsActiveState {
    () => {
        struct IsActiveState { platform : IsActivePlatform , attributes : gix_worktree :: Stack , }
    };
}

IsActiveState!();