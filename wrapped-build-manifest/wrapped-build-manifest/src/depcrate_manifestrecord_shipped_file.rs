// Generated macro for record_shipped_file (function)
macro_rules! Depcrate_manifestrecord_shipped_file {
() => {
// Module: crate::manifest
// Provides: {"record_shipped_file"}
// Dependencies: {}
fn record_shipped_file (builder : & mut Builder , path : PathBuf) -> Option < PathBuf > { if path . is_file () { builder . shipped_files . insert (path . file_name () . expect ("missing filename") . to_str () . expect ("non-utf-8 filename") . to_string () ,) ; Some (path) } else { None } }
};
}
