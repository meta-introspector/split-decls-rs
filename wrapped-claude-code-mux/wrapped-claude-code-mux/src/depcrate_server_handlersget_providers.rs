// Generated macro for get_providers (function)
macro_rules! Depcrate_server_handlersget_providers {
() => {
// Module: crate::server::handlers
// Provides: {"get_providers"}
// Dependencies: {}
# [doc = " Get providers configuration"] pub async fn get_providers (State (state) : State < Arc < AppState > >) -> impl IntoResponse { let config = state . config . read () . await ; Json (config . providers . clone ()) }
};
}
