// Generated macro for in_incr_comp_dir (function)
macro_rules! Depcrate_persist_fsin_incr_comp_dir {
() => {
// Module: crate::persist::fs
// Provides: {"in_incr_comp_dir"}
// Dependencies: {}
# [doc = " Returns the path for a given filename within the incremental compilation directory,"] # [doc = " not necessarily from the current session."] # [doc = ""] # [doc = " To ensure the file is part of the current session, use [`in_incr_comp_dir_sess`]."] pub fn in_incr_comp_dir (incr_comp_session_dir : & Path , file_name : & str) -> PathBuf { incr_comp_session_dir . join (file_name) }
};
}
