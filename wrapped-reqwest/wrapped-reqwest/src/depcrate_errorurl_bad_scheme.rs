// Generated macro for url_bad_scheme (function)
macro_rules! Depcrate_errorurl_bad_scheme {
() => {
// Module: crate::error
// Provides: {"url_bad_scheme"}
// Dependencies: {}
pub (crate) fn url_bad_scheme (url : Url) -> Error { Error :: new (Kind :: Builder , Some (BadScheme)) . with_url (url) }
};
}
