// Generated macro for default_on_ping (function)
macro_rules! Depcrate_http_websocketdefault_on_ping {
() => {
// Module: crate::http::websocket
// Provides: {"default_on_ping"}
// Dependencies: {}
# [doc = " Default ping handler function."] pub fn default_on_ping (_ : Option < & Data > , _ : Option < serde_json :: Value > ,) -> Ready < Result < Option < serde_json :: Value > > > { futures_util :: future :: ready (Ok (None)) }
};
}
