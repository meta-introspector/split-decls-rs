// Generated macro for StreamOrBuffer (enum)
macro_rules! Depcrate_client_blocking_io_http_curl_remoteStreamOrBuffer {
() => {
// Module: crate::client::blocking_io::http::curl::remote
// Provides: {"StreamOrBuffer"}
// Dependencies: {}
enum StreamOrBuffer { Stream (pipe :: Reader) , Buffer (std :: io :: Cursor < Vec < u8 > >) , }
};
}
