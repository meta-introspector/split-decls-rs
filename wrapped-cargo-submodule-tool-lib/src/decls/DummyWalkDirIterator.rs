macro_rules! DummyWalkDirIterator {
    () => {
        # [cfg (not (feature = "walkdir_enabled"))] pub struct DummyWalkDirIterator ;
    };
}

DummyWalkDirIterator!();