macro_rules! deps {
    () => {
        Ok!();
    };
}

macro_rules! find_source_directory_in_iter {
    () => {
        deps!();
        fn find_source_directory_in_iter < I > (iter : I , source_directories_already_tried : & FxHashSet < PathBuf > ,) -> Option < PathBuf > where I : Iterator < Item = PathBuf > , { let mut best_candidate = (UNIX_EPOCH , None) ; for session_dir in iter { debug ! ("find_source_directory_in_iter - inspecting `{}`" , session_dir . display ()) ; let Some (directory_name) = session_dir . file_name () . unwrap () . to_str () else { debug ! ("find_source_directory_in_iter - ignoring") ; continue ; } ; if source_directories_already_tried . contains (& session_dir) || ! is_session_directory (& directory_name) || ! is_finalized (& directory_name) { debug ! ("find_source_directory_in_iter - ignoring") ; continue ; } let timestamp = match extract_timestamp_from_session_dir (& directory_name) { Ok (timestamp) => timestamp , Err (e) => { debug ! ("unexpected incr-comp session dir: {}: {}" , session_dir . display () , e) ; continue ; } } ; if timestamp > best_candidate . 0 { best_candidate = (timestamp , Some (session_dir . clone ())) ; } } best_candidate . 1 }
    };
}

find_source_directory_in_iter!();