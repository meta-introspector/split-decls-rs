// Generated macro for oauth_logout (function)
macro_rules! Depcrate_server_oauth_handlersoauth_logout {
() => {
// Module: crate::server::oauth_handlers
// Provides: {"oauth_logout"}
// Dependencies: {}
pub async fn oauth_logout (State (app_state) : State < Arc < AppState > >) -> Redirect { app_state . token_store . clear_tokens () . await ; Redirect :: to ("/admin") }
};
}
