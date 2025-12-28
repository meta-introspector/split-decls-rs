macro_rules! deps {
    () => {
        DummyWalkDirIterator!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [cfg (not (feature = "walkdir_enabled"))] impl WalkDirIterator for DummyWalkDirIterator { fn new (_path : & Path) -> Self { DummyWalkDirIterator } fn into_iter (self) -> Box < dyn Iterator < Item = std :: result :: Result < PathBuf , String > > + Send > { Box :: new (std :: iter :: empty ()) } }
    };
}

impl_45!()