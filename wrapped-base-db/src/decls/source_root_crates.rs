macro_rules! deps {
    () => {
        RootQueryDb!();
    };
}

macro_rules! source_root_crates {
    () => {
        deps!();
        fn source_root_crates (db : & dyn RootQueryDb , id : SourceRootId) -> Arc < [Crate] > { let crates = db . all_crates () ; crates . iter () . copied () . filter (| & krate | { let root_file = krate . data (db) . root_file_id ; db . file_source_root (root_file) . source_root_id (db) == id }) . collect () }
    };
}

source_root_crates!()