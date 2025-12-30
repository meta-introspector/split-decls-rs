// Generated macro for base_url (function)
macro_rules! Depcrate_client_blocking_io_http_redirectbase_url {
() => {
// Module: crate::client::blocking_io::http::redirect
// Provides: {"base_url"}
// Dependencies: {}
pub (crate) fn base_url (redirect_url : & str , base_url : & str , url : String) -> Result < String , Error > { let tail = url . strip_prefix (base_url) . expect ("BUG: caller assures `base_url` is subset of `url`") ; redirect_url . strip_suffix (tail) . ok_or_else (| | Error { redirect_url : redirect_url . into () , expected_url : url , }) . map (ToOwned :: to_owned) }
};
}
