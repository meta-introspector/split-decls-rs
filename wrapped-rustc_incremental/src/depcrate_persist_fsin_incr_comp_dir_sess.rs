// Generated macro for in_incr_comp_dir_sess (function)
macro_rules! Depcrate_persist_fsin_incr_comp_dir_sess {
() => {
// Module: crate::persist::fs
// Provides: {"in_incr_comp_dir_sess"}
// Dependencies: {}
# [doc = " Returns the path for a given filename within the incremental compilation directory"] # [doc = " in the current session."] pub fn in_incr_comp_dir_sess (sess : & Session , file_name : & str) -> PathBuf { in_incr_comp_dir (& sess . incr_comp_session_dir () , file_name) }
};
}
