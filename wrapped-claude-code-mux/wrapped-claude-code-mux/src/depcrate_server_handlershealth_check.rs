// Generated macro for health_check (function)
macro_rules! Depcrate_server_handlershealth_check {
() => {
// Module: crate::server::handlers
// Provides: {"health_check"}
// Dependencies: {}
# [doc = " Health check endpoint"] pub async fn health_check () -> impl IntoResponse { Json (serde_json :: json ! ({ "status" : "ok" , "service" : "claude-code-mux" })) }
};
}
