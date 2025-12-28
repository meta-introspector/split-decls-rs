macro_rules! deps {
    () => {
        RealFileSystemStat!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl RealFileSystemStat { pub fn new (_git_executor : Arc < dyn GitExecutor + Send + Sync > , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : PathBuf ,) -> Self { RealFileSystemStat { } } }
    };
}

impl_93!();