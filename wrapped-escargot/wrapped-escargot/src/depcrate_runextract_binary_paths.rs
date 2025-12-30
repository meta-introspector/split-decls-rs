// Generated macro for extract_binary_paths (function)
macro_rules! Depcrate_runextract_binary_paths {
() => {
// Module: crate::run
// Provides: {"extract_binary_paths"}
// Dependencies: {}
fn extract_binary_paths (msgs : CommandMessages , kind : & 'static str ,) -> impl Iterator < Item = Result < path :: PathBuf , CargoError > > { msgs . filter_map (move | m | { let m = m . and_then (| m | { let m = m . decode () ? ; format :: log_message (& m) ; let p = extract_bin (& m , kind) . map (| p | p . to_path_buf ()) ; Ok (p) }) ; transpose (m) }) }
};
}
