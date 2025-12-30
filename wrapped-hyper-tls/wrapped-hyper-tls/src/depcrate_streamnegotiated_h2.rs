// Generated macro for negotiated_h2 (function)
macro_rules! Depcrate_streamnegotiated_h2 {
() => {
// Module: crate::stream
// Provides: {"negotiated_h2"}
// Dependencies: {}
# [cfg (feature = "alpn")] fn negotiated_h2 < T : std :: io :: Read + std :: io :: Write > (s : & native_tls :: TlsStream < T >) -> bool { s . negotiated_alpn () . unwrap_or (None) . map (| list | list == & b"h2" [..]) . unwrap_or (false) }
};
}
