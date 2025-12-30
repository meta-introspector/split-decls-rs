// Generated macro for new_html_response (function)
macro_rules! Depcratenew_html_response {
() => {
// Module: crate
// Provides: {"new_html_response"}
// Dependencies: {}
fn new_html_response (code : StatusCode) -> Response < String > { let mut resp = new_response (code) ; resp . headers_mut () . insert (header :: CONTENT_TYPE , HeaderValue :: from_static ("text/html; charset=utf-8") ,) ; resp }
};
}
