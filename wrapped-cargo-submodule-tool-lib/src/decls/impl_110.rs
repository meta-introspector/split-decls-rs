macro_rules! deps {
    () => {
        FileSystemWriter!();
        CachedFileSystemWriter!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl FileSystemWriter for CachedFileSystemWriter { fn write_file (& self , path : & Path , contents : & [u8]) -> Result < () > { fs :: write (path , contents) . with_context (| | format ! ("Failed to write file: {:?}" , path)) ? ; let mut rollup_lock = self . rollup_lock . lock () . unwrap () ; let metadata = self . file_system_stat . get_metadata (path) ? ; rollup_lock . file_metadata_cache . insert (path . to_path_buf () , metadata) ; Ok (()) } fn create_dir_all (& self , path : & Path) -> Result < () > { fs :: create_dir_all (path) . with_context (| | format ! ("Failed to create directory: {:?}" , path)) } fn remove_file (& self , path : & Path) -> Result < () > { fs :: remove_file (path) . with_context (| | format ! ("Failed to remove file: {:?}" , path)) ? ; let mut rollup_lock = self . rollup_lock . lock () . unwrap () ; rollup_lock . file_metadata_cache . remove (path) ; Ok (()) } fn remove_dir_all (& self , path : & Path) -> Result < () > { fs :: remove_dir_all (path) . with_context (| | format ! ("Failed to remove directory: {:?}" , path)) ? ; let mut rollup_lock = self . rollup_lock . lock () . unwrap () ; rollup_lock . file_metadata_cache . retain (| p , _ | ! p . starts_with (path)) ; Ok (()) } fn save_lock (& self) -> Result < () > { let rollup_lock = self . rollup_lock . lock () . unwrap () ; rollup_lock . save (& self . root_dir) } }
    };
}

impl_110!()