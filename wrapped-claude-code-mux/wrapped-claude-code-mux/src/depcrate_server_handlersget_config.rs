// Generated macro for get_config (function)
macro_rules! Depcrate_server_handlersget_config {
() => {
// Module: crate::server::handlers
// Provides: {"get_config"}
// Dependencies: {}
# [doc = " Get current routing configuration"] pub async fn get_config (State (state) : State < Arc < AppState > >) -> impl IntoResponse { let config = state . config . read () . await ; Json (get_base_config_json (& config)) }
};
}
