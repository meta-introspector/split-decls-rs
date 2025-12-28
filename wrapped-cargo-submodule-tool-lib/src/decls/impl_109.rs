macro_rules! deps {
    () => {
        FileSystemStat!();
        CachedFileSystemWriter!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl CachedFileSystemWriter { pub fn new (file_system_stat : Arc < dyn FileSystemStat > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf ,) -> Self { CachedFileSystemWriter { inner , rollup_lock , root_dir , } } }
    };
}

impl_109!();