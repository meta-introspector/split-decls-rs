macro_rules! deps {
    () => {
        FileSystemWriter!();
        CachedFileSystemWriter!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        # [cfg (not (feature = "git_enabled"))] impl FileSystemWriter for CachedFileSystemWriter { fn write_file (& self , path : & Path , _contents : & [u8]) -> Result < () > { println ! ("Dummy CachedFileSystemWriter: write_file to {:?}" , path) ; Ok (()) } fn create_dir_all (& self , path : & Path) -> Result < () > { println ! ("Dummy CachedFileSystemWriter: create_dir_all {:?}" , path) ; Ok (()) } fn remove_file (& self , path : & Path) -> Result < () > { println ! ("Dummy CachedFileSystemWriter: remove_file {:?}" , path) ; Ok (()) } fn remove_dir_all (& self , path : & Path) -> Result < () > { println ! ("Dummy CachedFileSystemWriter: remove_dir_all {:?}" , path) ; Ok (()) } fn save_lock (& self) -> Result < () > { println ! ("Dummy CachedFileSystemWriter: save_lock") ; Ok (()) } }
    };
}

impl_113!();