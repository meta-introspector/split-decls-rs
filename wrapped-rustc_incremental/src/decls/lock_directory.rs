macro_rules! deps {
    () => {
        CreateLock!();
        Ok!();
    };
}

macro_rules! lock_directory {
    () => {
        deps!();
        # [doc = " Allocate the lock-file and lock it."] fn lock_directory (sess : & Session , session_dir : & Path) -> (flock :: Lock , PathBuf) { let lock_file_path = lock_file_path (session_dir) ; debug ! ("lock_directory() - lock_file: {}" , lock_file_path . display ()) ; match flock :: Lock :: new (& lock_file_path , false , true , true ,) { Ok (lock) => (lock , lock_file_path) , Err (lock_err) => { let is_unsupported_lock = flock :: Lock :: error_unsupported (& lock_err) ; sess . dcx () . emit_fatal (errors :: CreateLock { lock_err , session_dir , is_unsupported_lock , is_cargo : rustc_session :: utils :: was_invoked_from_cargo () , }) ; } } }
    };
}

lock_directory!();