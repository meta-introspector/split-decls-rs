// Generated macro for EnteredTraceSpan (trait)
macro_rules! Depcrate_interpret_utilEnteredTraceSpan {
() => {
// Module: crate::interpret::util
// Provides: {"EnteredTraceSpan"}
// Dependencies: {}
# [doc = " A marker trait returned by [crate::interpret::Machine::enter_trace_span], identifying either a"] # [doc = " real [tracing::span::EnteredSpan] in case tracing is enabled, or the dummy type `()` when"] # [doc = " tracing is disabled. Also see [crate::enter_trace_span!] below."] pub trait EnteredTraceSpan { # [doc = " Allows executing an alternative function when tracing is disabled. Useful for example if you"] # [doc = " want to open a trace span when tracing is enabled, and alternatively just log a line when"] # [doc = " tracing is disabled."] fn or_if_tracing_disabled (self , f : impl FnOnce ()) -> Self ; }
};
}
