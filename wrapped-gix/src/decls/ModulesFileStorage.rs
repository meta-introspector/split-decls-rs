macro_rules! ModulesFileStorage {
    () => {
        pub (crate) type ModulesFileStorage = gix_features :: threading :: OwnShared < gix_fs :: SharedFileSnapshotMut < File > > ;
    };
}

ModulesFileStorage!();