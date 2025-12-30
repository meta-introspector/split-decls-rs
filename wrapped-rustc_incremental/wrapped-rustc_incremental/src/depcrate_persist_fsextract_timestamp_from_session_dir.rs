// Generated macro for extract_timestamp_from_session_dir (function)
macro_rules! Depcrate_persist_fsextract_timestamp_from_session_dir {
() => {
// Module: crate::persist::fs
// Provides: {"extract_timestamp_from_session_dir"}
// Dependencies: {}
fn extract_timestamp_from_session_dir (directory_name : & str) -> Result < SystemTime , & 'static str > { if ! is_session_directory (directory_name) { return Err ("not a directory") ; } let dash_indices : Vec < _ > = directory_name . match_indices ('-') . map (| (idx , _) | idx) . collect () ; if dash_indices . len () != 3 { return Err ("not three dashes in name") ; } string_to_timestamp (& directory_name [dash_indices [0] + 1 .. dash_indices [1]]) }
};
}
