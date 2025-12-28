macro_rules! deps {
    () => {
        RealWalkDirIterator!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        # [cfg (feature = "walkdir_enabled")] impl WalkDirIterator for RealWalkDirIterator { fn new (path : & Path) -> Self { RealWalkDirIterator { walkdir : WalkDir :: new (path) , } } fn into_iter (self) -> Box < dyn Iterator < Item = std :: result :: Result < PathBuf , String > > + Send > { Box :: new (self . walkdir . into_iter () . map (| entry_result | { entry_result . map (| entry | entry . path () . to_path_buf ()) . map_err (| e | e . to_string ()) })) } }
    };
}

impl_43!();