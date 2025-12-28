macro_rules! deps {
    () => {
        FileSystemStat!();
        CachedFileSystemStat!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl CachedFileSystemStat { pub fn new (inner : Arc < dyn FileSystemStat > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf ,) -> Self { CachedFileSystemStat { inner , rollup_lock , root_dir , } } }
    };
}

impl_97!();