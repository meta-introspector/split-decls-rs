// Generated macro for serve_admin (function)
macro_rules! Depcrate_server_handlersserve_admin {
() => {
// Module: crate::server::handlers
// Provides: {"serve_admin"}
// Dependencies: {}
# [doc = " Serve Admin UI"] pub async fn serve_admin () -> impl IntoResponse { Html (include_str ! ("admin.html")) }
};
}
