macro_rules! deps {
    () => {
        RealFileSystemStat!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl RealFileSystemStat { pub fn new (git_executor : Arc < dyn GitExecutor + Send + Sync > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf ,) -> Self { RealFileSystemStat { git_executor , rollup_lock , root_dir , } } }
    };
}

impl_89!();