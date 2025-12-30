// Generated macro for configure_tracing_subscriber (function)
macro_rules! Depcrateconfigure_tracing_subscriber {
() => {
// Module: crate
// Provides: {"configure_tracing_subscriber"}
// Dependencies: {}
pub fn configure_tracing_subscriber () { tracing :: subscriber :: set_global_default (tracing_subscriber :: FmtSubscriber :: builder () . with_env_filter (tracing_subscriber :: EnvFilter :: from_default_env ()) . finish () ,) . unwrap () ; }
};
}
