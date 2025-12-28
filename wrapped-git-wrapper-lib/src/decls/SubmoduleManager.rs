macro_rules! deps {
    () => {
        GitWrapperLibTrait!();
        RollupLock!();
    };
}

macro_rules! SubmoduleManager {
    () => {
        deps!();
        pub struct SubmoduleManager { git_wrapper : Arc < dyn GitWrapperLibTrait + Send + Sync > , rollup_lock : Arc < Mutex < RollupLock > > , root_dir : PathBuf , }
    };
}

SubmoduleManager!()