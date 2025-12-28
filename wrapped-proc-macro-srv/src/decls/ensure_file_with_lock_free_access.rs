macro_rules! ensure_file_with_lock_free_access {
    () => {
        # [cfg (unix)] fn ensure_file_with_lock_free_access (_temp_dir : & TempDir , path : & Utf8Path ,) -> io :: Result < Utf8PathBuf > { Ok (path . to_owned ()) }
    };
}

ensure_file_with_lock_free_access!()