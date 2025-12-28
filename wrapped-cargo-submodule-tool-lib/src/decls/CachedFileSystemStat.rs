macro_rules! CachedFileSystemStat {
    () => {
        # [cfg (not (feature = "git_enabled"))] pub struct CachedFileSystemStat ;
    };
}

CachedFileSystemStat!()