// Generated macro for up_to_date (function)
macro_rules! Depcrate_utils_helpersup_to_date {
() => {
// Module: crate::utils::helpers
// Provides: {"up_to_date"}
// Dependencies: {}
# [doc = " Returns `true` if `dst` is up to date given that the file or files in `src`"] # [doc = " are used to generate it."] # [doc = ""] # [doc = " Uses last-modified time checks to verify this."] pub fn up_to_date (src : & Path , dst : & Path) -> bool { if ! dst . exists () { return false ; } let threshold = mtime (dst) ; let meta = match fs :: metadata (src) { Ok (meta) => meta , Err (e) => panic ! ("source {src:?} failed to get metadata: {e}") , } ; if meta . is_dir () { dir_up_to_date (src , threshold) } else { meta . modified () . unwrap_or (UNIX_EPOCH) <= threshold } }
};
}
