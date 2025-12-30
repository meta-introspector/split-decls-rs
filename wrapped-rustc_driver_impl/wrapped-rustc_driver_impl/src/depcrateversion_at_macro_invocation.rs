// Generated macro for version_at_macro_invocation (function)
macro_rules! Depcrateversion_at_macro_invocation {
() => {
// Module: crate
// Provides: {"version_at_macro_invocation"}
// Dependencies: {}
# [doc (hidden)] pub fn version_at_macro_invocation (early_dcx : & EarlyDiagCtxt , binary : & str , matches : & getopts :: Matches , version : & str , commit_hash : & str , commit_date : & str , release : & str ,) { let verbose = matches . opt_present ("verbose") ; let mut version = version ; let mut release = release ; let tmp ; if let Ok (force_version) = std :: env :: var ("RUSTC_OVERRIDE_VERSION_STRING") { tmp = force_version ; version = & tmp ; release = & tmp ; } safe_println ! ("{binary} {version}") ; if verbose { safe_println ! ("binary: {binary}") ; safe_println ! ("commit-hash: {commit_hash}") ; safe_println ! ("commit-date: {commit_date}") ; safe_println ! ("host: {}" , config :: host_tuple ()) ; safe_println ! ("release: {release}") ; get_backend_from_raw_matches (early_dcx , matches) . print_version () ; } }
};
}
