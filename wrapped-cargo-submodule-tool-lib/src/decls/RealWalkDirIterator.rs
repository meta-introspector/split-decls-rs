macro_rules! RealWalkDirIterator {
    () => {
        # [cfg (feature = "walkdir_enabled")] pub struct RealWalkDirIterator { walkdir : WalkDir , }
    };
}

RealWalkDirIterator!()