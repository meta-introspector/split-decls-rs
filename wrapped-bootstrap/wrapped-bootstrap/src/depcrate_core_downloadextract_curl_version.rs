// Generated macro for extract_curl_version (function)
macro_rules! Depcrate_core_downloadextract_curl_version {
() => {
// Module: crate::core::download
// Provides: {"extract_curl_version"}
// Dependencies: {}
fn extract_curl_version (out : String) -> semver :: Version { out . lines () . next () . and_then (| line | line . split (" ") . nth (1)) . and_then (| version | semver :: Version :: parse (version) . ok ()) . unwrap_or (semver :: Version :: new (1 , 0 , 0)) }
};
}
