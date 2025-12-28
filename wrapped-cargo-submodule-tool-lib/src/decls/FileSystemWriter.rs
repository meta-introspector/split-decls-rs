macro_rules! FileSystemWriter {
    () => {
        pub trait FileSystemWriter { fn write_file (& self , path : & Path , contents : & [u8]) -> Result < () > ; fn create_dir_all (& self , path : & Path) -> Result < () > ; fn remove_file (& self , path : & Path) -> Result < () > ; fn remove_dir_all (& self , path : & Path) -> Result < () > ; fn save_lock (& self) -> Result < () > ; }
    };
}

FileSystemWriter!();