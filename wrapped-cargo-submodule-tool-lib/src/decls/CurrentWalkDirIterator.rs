macro_rules! deps {
    () => {
        DummyWalkDirIterator!();
    };
}

macro_rules! CurrentWalkDirIterator {
    () => {
        deps!();
        # [cfg (not (feature = "walkdir_enabled"))] pub type CurrentWalkDirIterator = DummyWalkDirIterator ;
    };
}

CurrentWalkDirIterator!()