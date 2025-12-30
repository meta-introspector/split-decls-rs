// Generated macro for DefaultOnPingType (type)
macro_rules! Depcrate_http_websocketDefaultOnPingType {
() => {
// Module: crate::http::websocket
// Provides: {"DefaultOnPingType"}
// Dependencies: {}
# [doc = " Default ping handler type."] pub type DefaultOnPingType = fn (Option < & Data > , Option < serde_json :: Value >) -> Ready < Result < Option < serde_json :: Value > > > ;
};
}
