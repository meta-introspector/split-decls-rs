macro_rules! deps {
    () => {
        DeleteLock!();
    };
}

macro_rules! delete_session_dir_lock_file {
    () => {
        deps!();
        fn delete_session_dir_lock_file (sess : & Session , lock_file_path : & Path) { if let Err (err) = safe_remove_file (lock_file_path) { sess . dcx () . emit_warn (errors :: DeleteLock { path : lock_file_path , err }) ; } }
    };
}

delete_session_dir_lock_file!();