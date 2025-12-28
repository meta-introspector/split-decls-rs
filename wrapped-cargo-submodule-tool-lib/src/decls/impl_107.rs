macro_rules! deps {
    () => {
        RealFileSystemWriter!();
        FileSystemWriter!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl FileSystemWriter for RealFileSystemWriter { fn write_file (& self , path : & Path , contents : & [u8]) -> Result < () > { fs :: write (path , contents) . with_context (| | format ! ("Failed to write file: {:?}" , path)) } fn create_dir_all (& self , path : & Path) -> Result < () > { fs :: create_dir_all (path) . with_context (| | format ! ("Failed to create directory: {:?}" , path)) } fn remove_file (& self , path : & Path) -> Result < () > { fs :: remove_file (path) . with_context (| | format ! ("Failed to remove file: {:?}" , path)) } fn remove_dir_all (& self , path : & Path) -> Result < () > { fs :: remove_dir_all (path) . with_context (| | format ! ("Failed to remove directory: {:?}" , path)) } fn save_lock (& self) -> Result < () > { Ok (()) } }
    };
}

impl_107!()