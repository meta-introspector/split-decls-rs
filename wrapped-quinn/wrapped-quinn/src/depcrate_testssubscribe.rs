// Generated macro for subscribe (function)
macro_rules! Depcrate_testssubscribe {
() => {
// Module: crate::tests
// Provides: {"subscribe"}
// Dependencies: {}
fn subscribe () -> tracing :: subscriber :: DefaultGuard { let sub = tracing_subscriber :: FmtSubscriber :: builder () . with_env_filter (EnvFilter :: from_default_env ()) . with_writer (| | TestWriter) . finish () ; tracing :: subscriber :: set_default (sub) }
};
}
