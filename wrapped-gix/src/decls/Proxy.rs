macro_rules! deps {
    () => {
        Clone!();
        Repository!();
    };
}

macro_rules! Proxy {
    () => {
        deps!();
        # [doc = " A stand-in to a worktree as result of a worktree iteration."] # [doc = ""] # [doc = " It provides access to typical worktree state, but may not actually point to a valid checkout as the latter has been moved or"] # [doc = " deleted."] # [derive (Debug , Clone)] pub struct Proxy < 'repo > { pub (crate) parent : & 'repo Repository , pub (crate) git_dir : PathBuf , }
    };
}

Proxy!();