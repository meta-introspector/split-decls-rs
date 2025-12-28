macro_rules! lock_file_path {
    () => {
        # [doc = " Locks a given session directory."] fn lock_file_path (session_dir : & Path) -> PathBuf { let crate_dir = session_dir . parent () . unwrap () ; let directory_name = session_dir . file_name () . unwrap () . to_str () . expect ("malformed session dir name: contains non-Unicode characters") ; let dash_indices : Vec < _ > = directory_name . match_indices ('-') . map (| (idx , _) | idx) . collect () ; if dash_indices . len () != 3 { bug ! ("Encountered incremental compilation session directory with \
              malformed name: {}" , session_dir . display ()) } crate_dir . join (& directory_name [0 .. dash_indices [2]]) . with_extension (& LOCK_FILE_EXT [1 ..]) }
    };
}

lock_file_path!()