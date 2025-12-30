// Generated macro for get_models_config (function)
macro_rules! Depcrate_server_handlersget_models_config {
() => {
// Module: crate::server::handlers
// Provides: {"get_models_config"}
// Dependencies: {}
# [doc = " Get models configuration"] pub async fn get_models_config (State (state) : State < Arc < AppState > >) -> impl IntoResponse { let config = state . config . read () . await ; Json (config . models . clone ()) }
};
}
