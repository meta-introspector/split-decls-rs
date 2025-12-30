// Generated macro for extract_binary_paths (function)
macro_rules! Depcrate_testextract_binary_paths {
() => {
// Module: crate::test
// Provides: {"extract_binary_paths"}
// Dependencies: {}
fn extract_binary_paths (msgs : CommandMessages ,) -> impl Iterator < Item = Result < CargoTest , CargoError > > { msgs . filter_map (move | m | { let m = m . and_then (| m | { let m = m . decode () ? ; format :: log_message (& m) ; let p = extract_bin (& m) ; Ok (p) }) ; transpose (m) }) }
};
}
