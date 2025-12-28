macro_rules! CommitsStorage {
    () => {
        pub (crate) type CommitsStorage = gix_features :: threading :: OwnShared < gix_fs :: SharedFileSnapshotMut < Vec < gix_hash :: ObjectId > > > ;
    };
}

CommitsStorage!()