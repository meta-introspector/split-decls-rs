macro_rules! try_unlink_path_recursively {
    () => {
        fn try_unlink_path_recursively (path : & Path , path_meta : & std :: fs :: Metadata) -> std :: io :: Result < () > { if path_meta . is_dir () { std :: fs :: remove_dir_all (path) } else if path_meta . file_type () . is_symlink () { gix_fs :: symlink :: remove (path) } else { std :: fs :: remove_file (path) } }
    };
}

try_unlink_path_recursively!();