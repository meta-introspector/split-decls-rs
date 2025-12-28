macro_rules! IndexStorage {
    () => {
        # [cfg (feature = "index")] pub (crate) type IndexStorage = gix_features :: threading :: OwnShared < gix_fs :: SharedFileSnapshotMut < gix_index :: File > > ;
    };
}

IndexStorage!()