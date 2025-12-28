macro_rules! deps {
    () => {
        RollupLock!();
    };
}

macro_rules! PureRustGitExecutor {
    () => {
        deps!();
        pub struct PureRustGitExecutor { rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf , }
    };
}

PureRustGitExecutor!();