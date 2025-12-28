macro_rules! FileSystemStat {
    () => {
        pub trait FileSystemStat : Send + Sync { fn get_metadata (& self , path : & Path) -> Result < FileMetadata > ; }
    };
}

FileSystemStat!();