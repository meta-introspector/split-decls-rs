// Generated macro for shutdown_server (function)
macro_rules! Depcrate_servershutdown_server {
() => {
// Module: crate::server
// Provides: {"shutdown_server"}
// Dependencies: {}
async fn shutdown_server (State (_app_state) : State < Arc < AppState > >) -> impl IntoResponse { info ! ("Shutting down server...") ; tokio :: spawn (async move { tokio :: time :: sleep (tokio :: time :: Duration :: from_millis (500)) . await ; std :: process :: exit (0) ; }) ; (StatusCode :: OK , Html ("<div class='px-4 py-3 rounded-xl bg-primary/20 border border-primary/50 text-foreground text-sm'>✅ Server shutting down...</div>" . to_string ())) }
};
}
