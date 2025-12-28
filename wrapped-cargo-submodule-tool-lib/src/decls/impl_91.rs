macro_rules! deps {
    () => {
        RealFileSystemStat!();
        FileSystemStat!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [cfg (feature = "git_enabled")] impl FileSystemStat for RealFileSystemStat { fn get_metadata (& self , path : & Path) -> Result < FileMetadata > { let metadata = fs :: metadata (path) ? ; let file_content = fs :: read (path) ? ; let hash = calculate_file_hash (& file_content) ; let (is_git_tracked , git_object_hash) = self . git_executor . get_file_git_info (& self . root_dir , path) ? ; Ok (FileMetadata { modified : metadata . modified () ? , len : metadata . len () , hash , git_object_hash , is_git_tracked , }) } }
    };
}

impl_91!();