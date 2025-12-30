// Generated macro for decode_into_cstring (function)
macro_rules! Depcrate_mysql_connection_urldecode_into_cstring {
() => {
// Module: crate::mysql::connection::url
// Provides: {"decode_into_cstring"}
// Dependencies: {}
fn decode_into_cstring (s : & str) -> ConnectionResult < CString > { let decoded = percent_decode (s . as_bytes ()) . decode_utf8 () . map_err (| _ | connection_url_error ()) ? ; CString :: new (decoded . as_bytes ()) . map_err (Into :: into) }
};
}
