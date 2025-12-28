macro_rules! strip_lock_suffix {
    () => {
        fn strip_lock_suffix (lock_path : & Path) -> PathBuf { let ext = lock_path . extension () . expect ("at least our own extension") . to_str () . expect ("no illegal UTF8 in extension") ; lock_path . with_extension (ext . split_at (ext . len () . saturating_sub (DOT_LOCK_SUFFIX . len ())) . 0) }
    };
}

strip_lock_suffix!();