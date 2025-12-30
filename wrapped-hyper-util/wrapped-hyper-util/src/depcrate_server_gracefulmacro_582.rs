// Generated macro for macro_582 (macro)
macro_rules! Depcrate_server_gracefulmacro_582 {
() => {
// Module: crate::server::graceful
// Provides: {"macro_582"}
// Dependencies: {}
pin_project ! { struct GracefulConnectionFuture < C , F : Future > { # [pin] conn : C , # [pin] cancel : F , # [pin] cancelled_guard : Option < F :: Output >, } }
};
}
