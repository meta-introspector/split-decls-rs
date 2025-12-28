macro_rules! is_session_directory {
    () => {
        fn is_session_directory (directory_name : & str) -> bool { directory_name . starts_with ("s-") && ! directory_name . ends_with (LOCK_FILE_EXT) }
    };
}

is_session_directory!();