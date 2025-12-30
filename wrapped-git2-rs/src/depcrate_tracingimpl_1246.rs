// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_tracingimpl_1246 {
() => {
// Module: crate::tracing
// Provides: {"impl_1246"}
// Dependencies: {}
impl Binding for TraceLevel { type Raw = raw :: git_trace_level_t ; unsafe fn from_raw (raw : raw :: git_trace_level_t) -> Self { match raw { raw :: GIT_TRACE_NONE => Self :: None , raw :: GIT_TRACE_FATAL => Self :: Fatal , raw :: GIT_TRACE_ERROR => Self :: Error , raw :: GIT_TRACE_WARN => Self :: Warn , raw :: GIT_TRACE_INFO => Self :: Info , raw :: GIT_TRACE_DEBUG => Self :: Debug , raw :: GIT_TRACE_TRACE => Self :: Trace , _ => panic ! ("Unknown git trace level") , } } fn raw (& self) -> raw :: git_trace_level_t { match * self { Self :: None => raw :: GIT_TRACE_NONE , Self :: Fatal => raw :: GIT_TRACE_FATAL , Self :: Error => raw :: GIT_TRACE_ERROR , Self :: Warn => raw :: GIT_TRACE_WARN , Self :: Info => raw :: GIT_TRACE_INFO , Self :: Debug => raw :: GIT_TRACE_DEBUG , Self :: Trace => raw :: GIT_TRACE_TRACE , } } }
};
}
