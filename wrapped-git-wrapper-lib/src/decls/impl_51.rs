macro_rules! deps {
    () => {
        PureRustGitExecutor!();
        RollupLock!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl PureRustGitExecutor { pub fn new (rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf) -> Self { PureRustGitExecutor { rollup_lock , root_dir , } } }
    };
}

impl_51!();