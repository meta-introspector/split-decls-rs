macro_rules! deps {
    () => {
        Ok!();
    };
}

macro_rules! delete_all_session_dir_contents {
    () => {
        deps!();
        pub (crate) fn delete_all_session_dir_contents (sess : & Session) -> io :: Result < () > { let sess_dir_iterator = sess . incr_comp_session_dir () . read_dir () ? ; for entry in sess_dir_iterator { let entry = entry ? ; safe_remove_file (& entry . path ()) ? } Ok (()) }
    };
}

delete_all_session_dir_contents!();