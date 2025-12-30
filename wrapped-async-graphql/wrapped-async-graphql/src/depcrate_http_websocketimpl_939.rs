// Generated macro for impl_939 (impl)
macro_rules! Depcrate_http_websocketimpl_939 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_939"}
// Dependencies: {}
impl < S , E > WebSocket < MessageMapStream < S > , E , DefaultOnConnInitType , DefaultOnPingType > where E : Executor , S : Stream , S :: Item : AsRef < [u8] > , { # [doc = " Create a new websocket from bytes stream."] pub fn new (executor : E , stream : S , protocol : Protocols) -> Self { let stream = stream . map (ClientMessage :: from_bytes as fn (S :: Item) -> serde_json :: Result < ClientMessage >) ; WebSocket :: from_message_stream (executor , stream , protocol) } }
};
}
