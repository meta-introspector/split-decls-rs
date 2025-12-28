macro_rules! is_session_directory_lock_file {
    () => {
        fn is_session_directory_lock_file (file_name : & str) -> bool { file_name . starts_with ("s-") && file_name . ends_with (LOCK_FILE_EXT) }
    };
}

is_session_directory_lock_file!();