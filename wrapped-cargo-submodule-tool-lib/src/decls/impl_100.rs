macro_rules! deps {
    () => {
        FileSystemStat!();
        CachedFileSystemStat!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl CachedFileSystemStat { pub fn new (_inner : Arc < dyn FileSystemStat > , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : PathBuf ,) -> Self { CachedFileSystemStat { } } }
    };
}

impl_100!();