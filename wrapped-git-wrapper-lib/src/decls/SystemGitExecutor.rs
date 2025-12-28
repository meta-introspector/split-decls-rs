macro_rules! deps {
    () => {
        Execv!();
        RollupLock!();
    };
}

macro_rules! SystemGitExecutor {
    () => {
        deps!();
        pub struct SystemGitExecutor { git_executable_path : PathBuf , executor : Arc < dyn Execv + Send + Sync > , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : PathBuf , }
    };
}

SystemGitExecutor!();