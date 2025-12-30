// Generated macro for url_invalid_uri (function)
macro_rules! Depcrate_errorurl_invalid_uri {
() => {
// Module: crate::error
// Provides: {"url_invalid_uri"}
// Dependencies: {}
pub (crate) fn url_invalid_uri (url : Url) -> Error { Error :: new (Kind :: Builder , Some ("Parsed Url is not a valid Uri")) . with_url (url) }
};
}
