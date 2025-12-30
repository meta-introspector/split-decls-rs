// Generated macro for TracingCb (type)
macro_rules! Depcrate_tracingTracingCb {
() => {
// Module: crate::tracing
// Provides: {"TracingCb"}
// Dependencies: {}
# [doc = " Callback type used to pass tracing events to the subscriber."] # [doc = " see `trace_set` to register a subscriber."] pub type TracingCb = fn (TraceLevel , & [u8]) ;
};
}
