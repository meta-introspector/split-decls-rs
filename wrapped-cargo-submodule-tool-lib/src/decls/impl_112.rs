macro_rules! deps {
    () => {
        CachedFileSystemWriter!();
        FileSystemStat!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl CachedFileSystemWriter { pub fn new (_file_system_stat : Arc < dyn FileSystemStat > , _rollup_lock : Arc < Mutex < RollupLock > > , _root_dir : PathBuf ,) -> Self { CachedFileSystemWriter { } } }
    };
}

impl_112!();