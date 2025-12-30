// Generated macro for map_header (function)
macro_rules! Depcrate_recordreplay_qlogmap_header {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"map_header"}
// Dependencies: {}
fn map_header (hdr : & HttpHeader , host_override : Option < & str > ,) -> quiche :: h3 :: Header { if hdr . name . eq_ignore_ascii_case (":authority") || hdr . name . eq_ignore_ascii_case ("host") { if let Some (host) = host_override { return quiche :: h3 :: Header :: new (hdr . name . as_bytes () , host . as_bytes ()) ; } } quiche :: h3 :: Header :: new (hdr . name . as_bytes () , hdr . value . as_bytes ()) }
};
}
