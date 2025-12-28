macro_rules! in_incr_comp_dir {
    () => {
        # [doc = " Returns the path for a given filename within the incremental compilation directory,"] # [doc = " not necessarily from the current session."] # [doc = ""] # [doc = " To ensure the file is part of the current session, use [`in_incr_comp_dir_sess`]."] pub fn in_incr_comp_dir (incr_comp_session_dir : & Path , file_name : & str) -> PathBuf { incr_comp_session_dir . join (file_name) }
    };
}

in_incr_comp_dir!();