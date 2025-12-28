macro_rules! deps {
    () => {
        RealFileSystemStat!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl Clone for RealFileSystemStat { fn clone (& self) -> Self { RealFileSystemStat { } } }
    };
}

impl_94!()