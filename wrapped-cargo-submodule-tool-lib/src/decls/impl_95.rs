macro_rules! deps {
    () => {
        FileSystemStat!();
        RealFileSystemStat!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl FileSystemStat for RealFileSystemStat { fn get_metadata (& self , path : & Path) -> Result < FileMetadata > { println ! ("Dummy RealFileSystemStat: get_metadata for {:?}" , path) ; Ok (FileMetadata :: default ()) } }
    };
}

impl_95!()