macro_rules! deps {
    () => {
        CachedFileSystemStat!();
        FileSystemStat!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl FileSystemStat for CachedFileSystemStat { fn get_metadata (& self , path : & Path) -> Result < FileMetadata > { println ! ("Dummy CachedFileSystemStat: get_metadata for {:?}" , path) ; Ok (FileMetadata :: default ()) } }
    };
}

impl_101!();