// Generated macro for default_on_connection_init (function)
macro_rules! Depcrate_http_websocketdefault_on_connection_init {
() => {
// Module: crate::http::websocket
// Provides: {"default_on_connection_init"}
// Dependencies: {}
# [doc = " Default connection initializer function."] pub fn default_on_connection_init (_ : serde_json :: Value) -> Ready < Result < Data > > { futures_util :: future :: ready (Ok (Data :: default ())) }
};
}
