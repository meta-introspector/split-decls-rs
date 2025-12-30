// Generated macro for append_url (function)
macro_rules! Depcrate_client_blocking_io_httpappend_url {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"append_url"}
// Dependencies: {}
fn append_url (base : & str , suffix : & str) -> String { let mut buf = base . to_owned () ; if base . as_bytes () . last () != Some (& b'/') { buf . push ('/') ; } buf . push_str (suffix) ; buf }
};
}
