macro_rules! RealFileSystemStat {
    () => {
        # [cfg (not (feature = "git_enabled"))] pub struct RealFileSystemStat ;
    };
}

RealFileSystemStat!()