macro_rules! deps {
    () => {
        FileSystemStat!();
        CachedFileSystemStat!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl FileSystemStat for CachedFileSystemStat { fn get_metadata (& self , path : & Path) -> Result < FileMetadata > { let mut rollup_lock_guard = self . rollup_lock . lock () . unwrap () ; let relative_path = path . strip_prefix (& self . root_dir) . unwrap_or (path) ; if let Some (metadata) = rollup_lock_guard . get_metadata (relative_path) { let current_metadata = self . inner . get_metadata (path) ? ; if current_metadata == * metadata { return Ok (metadata . clone ()) ; } } let metadata = self . inner . get_metadata (path) ? ; rollup_lock_guard . set_metadata (relative_path . to_path_buf () , metadata . clone ()) ; Ok (metadata) } }
    };
}

impl_98!();