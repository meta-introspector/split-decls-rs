// Generated macro for dir_up_to_date (function)
macro_rules! Depcrate_utils_helpersdir_up_to_date {
() => {
// Module: crate::utils::helpers
// Provides: {"dir_up_to_date"}
// Dependencies: {}
fn dir_up_to_date (src : & Path , threshold : SystemTime) -> bool { t ! (fs :: read_dir (src)) . map (| e | t ! (e)) . all (| e | { let meta = t ! (e . metadata ()) ; if meta . is_dir () { dir_up_to_date (& e . path () , threshold) } else { meta . modified () . unwrap_or (UNIX_EPOCH) < threshold } }) }
};
}
