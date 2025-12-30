// Generated macro for delete_all_session_dir_contents (function)
macro_rules! Depcrate_persist_fsdelete_all_session_dir_contents {
() => {
// Module: crate::persist::fs
// Provides: {"delete_all_session_dir_contents"}
// Dependencies: {}
pub (crate) fn delete_all_session_dir_contents (sess : & Session) -> io :: Result < () > { let sess_dir_iterator = sess . incr_comp_session_dir () . read_dir () ? ; for entry in sess_dir_iterator { let entry = entry ? ; safe_remove_file (& entry . path ()) ? } Ok (()) }
};
}
