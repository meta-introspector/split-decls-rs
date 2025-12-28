macro_rules! deps {
    () => {
        RealFileSystemStat!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl Clone for RealFileSystemStat { fn clone (& self) -> Self { RealFileSystemStat { git_executor : self . git_executor . clone () , rollup_lock : self . rollup_lock . clone () , root_dir : self . root_dir . clone () , } } }
    };
}

impl_90!()