macro_rules! deps {
    () => {
        GitExecutor!();
        Result!();
        RollupLock!();
    };
}

macro_rules! create_snapshot {
    () => {
        deps!();
        pub fn create_snapshot (_root_dir : & Path , _rollup_lock : Arc < Mutex < RollupLock > > , _git_executor : Arc < dyn GitExecutor + Send + Sync > ,) -> Result < () > { println ! ("create_snapshot called (placeholder)") ; Ok (()) }
    };
}

create_snapshot!()