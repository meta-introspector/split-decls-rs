// Generated macro for impl_938 (impl)
macro_rules! Depcrate_http_websocketimpl_938 {
() => {
// Module: crate::http::websocket
// Provides: {"impl_938"}
// Dependencies: {}
impl < S , E > WebSocket < S , E , DefaultOnConnInitType , DefaultOnPingType > where E : Executor , S : Stream < Item = serde_json :: Result < ClientMessage > > , { # [doc = " Create a new websocket from [`ClientMessage`] stream."] pub fn from_message_stream (executor : E , stream : S , protocol : Protocols) -> Self { WebSocket { on_connection_init : Some (default_on_connection_init) , on_ping : default_on_ping , init_fut : None , ping_fut : None , connection_data : None , data : None , executor , streams : HashMap :: new () , stream , protocol , last_msg_at : Instant :: now () , keepalive_timer : None , close : false , } } }
};
}
