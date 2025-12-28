macro_rules! deps {
    () => {
        SessionGcFailed!();
    };
}

macro_rules! delete_old {
    () => {
        deps!();
        fn delete_old (sess : & Session , path : & Path) { debug ! ("garbage_collect_session_directories() - deleting `{}`" , path . display ()) ; if let Err (err) = std_fs :: remove_dir_all (path) { sess . dcx () . emit_warn (errors :: SessionGcFailed { path , err }) ; } else { delete_session_dir_lock_file (sess , & lock_file_path (path)) ; } }
    };
}

delete_old!();