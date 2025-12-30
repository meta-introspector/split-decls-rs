// Generated macro for MessageMapStream (type)
macro_rules! Depcrate_http_websocketMessageMapStream {
() => {
// Module: crate::http::websocket
// Provides: {"MessageMapStream"}
// Dependencies: {}
type MessageMapStream < S > = futures_util :: stream :: Map < S , fn (< S as Stream > :: Item) -> serde_json :: Result < ClientMessage > > ;
};
}
