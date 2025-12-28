macro_rules! in_incr_comp_dir_sess {
    () => {
        # [doc = " Returns the path for a given filename within the incremental compilation directory"] # [doc = " in the current session."] pub fn in_incr_comp_dir_sess (sess : & Session , file_name : & str) -> PathBuf { in_incr_comp_dir (& sess . incr_comp_session_dir () , file_name) }
    };
}

in_incr_comp_dir_sess!();