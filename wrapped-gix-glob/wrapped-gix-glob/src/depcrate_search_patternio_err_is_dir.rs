// Generated macro for io_err_is_dir (function)
macro_rules! Depcrate_search_patternio_err_is_dir {
() => {
// Module: crate::search::pattern
// Provides: {"io_err_is_dir"}
// Dependencies: {}
fn io_err_is_dir (err : & std :: io :: Error) -> bool { let raw = err . raw_os_error () ; raw == Some (if cfg ! (windows) { 5 } else { 21 }) || raw == Some (20) }
};
}
