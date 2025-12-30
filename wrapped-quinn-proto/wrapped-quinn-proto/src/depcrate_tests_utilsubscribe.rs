// Generated macro for subscribe (function)
macro_rules! Depcrate_tests_utilsubscribe {
() => {
// Module: crate::tests::util
// Provides: {"subscribe"}
// Dependencies: {}
pub (super) fn subscribe () -> tracing :: subscriber :: DefaultGuard { let builder = tracing_subscriber :: FmtSubscriber :: builder () . with_max_level (tracing :: Level :: TRACE) . with_writer (| | TestWriter) ; # [cfg (all (target_family = "wasm" , target_os = "unknown"))] let builder = builder . without_time () ; tracing :: subscriber :: set_default (builder . finish ()) }
};
}
