// Generated macro for curl_version (function)
macro_rules! Depcrate_core_downloadcurl_version {
() => {
// Module: crate::core::download
// Provides: {"curl_version"}
// Dependencies: {}
fn curl_version (exec_ctx : & ExecutionContext) -> semver :: Version { let mut curl = command ("curl") ; curl . arg ("-V") ; let curl = curl . run_capture_stdout (exec_ctx) ; if curl . is_failure () { return semver :: Version :: new (1 , 0 , 0) ; } let output = curl . stdout () ; extract_curl_version (output) }
};
}
