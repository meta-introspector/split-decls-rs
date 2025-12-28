macro_rules! deps {
    () => {
        RollupLock!();
        Result!();
    };
}

macro_rules! create_snapshot_without_executor {
    () => {
        deps!();
        pub fn create_snapshot_without_executor (_root_dir : & Path , _rollup_lock : Arc < Mutex < RollupLock > > ,) -> Result < () > { println ! ("create_snapshot_without_executor called (placeholder)") ; Ok (()) }
    };
}

create_snapshot_without_executor!()