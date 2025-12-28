macro_rules! deps {
    () => {
        RollupLock!();
        SystemGitExecutor!();
        Execv!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl SystemGitExecutor { pub fn new (git_executable_path : PathBuf , executor : Arc < dyn Execv + Send + Sync > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf ,) -> Self { SystemGitExecutor { git_executable_path , executor , _rollup_lock : rollup_lock , _root_dir : root_dir , } } }
    };
}

impl_89!();