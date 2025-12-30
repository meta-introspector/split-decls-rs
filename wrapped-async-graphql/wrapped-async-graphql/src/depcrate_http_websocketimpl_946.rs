// Generated macro for impl_946 (impl)
macro_rules! Depcrate_http_websocketimpl_946 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_946"}
// Dependencies: {}
impl ClientMessage { # [doc = " Creates a ClientMessage from an array of bytes"] pub fn from_bytes < T > (message : T) -> serde_json :: Result < Self > where T : AsRef < [u8] > , { serde_json :: from_slice (message . as_ref ()) } }
};
}
