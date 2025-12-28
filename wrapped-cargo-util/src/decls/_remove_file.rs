macro_rules! _remove_file {
    () => {
        fn _remove_file (p : & Path) -> Result < () > { # [cfg (target_os = "windows")] { use std :: os :: windows :: fs :: FileTypeExt ; let metadata = symlink_metadata (p) ? ; let file_type = metadata . file_type () ; if file_type . is_symlink_dir () { return remove_symlink_dir_with_permission_check (p) ; } } remove_file_with_permission_check (p) }
    };
}

_remove_file!()