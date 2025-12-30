// Generated macro for find_tz_file (function)
macro_rules! Depcrate_offset_local_tz_info_timezonefind_tz_file {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"find_tz_file"}
// Dependencies: {}
# [doc = " Open the TZif file corresponding to a TZ string"] fn find_tz_file (path : impl AsRef < Path >) -> Result < File , Error > { # [cfg (not (unix))] return Ok (File :: open (path) ?) ; # [cfg (unix)] { let path = path . as_ref () ; if path . is_absolute () { return Ok (File :: open (path) ?) ; } for folder in & ZONE_INFO_DIRECTORIES { if let Ok (file) = File :: open (PathBuf :: from (folder) . join (path)) { return Ok (file) ; } } Err (Error :: Io (io :: ErrorKind :: NotFound . into ())) } }
};
}
