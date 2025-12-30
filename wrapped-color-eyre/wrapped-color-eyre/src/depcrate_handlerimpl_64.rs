// Generated macro for impl_64 (impl)
macro_rules! Depcrate_handlerimpl_64 {
() => {
// Module: crate::handler
// Provides: {"impl_64"}
// Dependencies: {}
impl Handler { # [doc = " Return a reference to the captured `Backtrace` type"] pub fn backtrace (& self) -> Option < & Backtrace > { self . backtrace . as_ref () } # [doc = " Return a reference to the captured `SpanTrace` type"] # [cfg (feature = "capture-spantrace")] # [cfg_attr (docsrs , doc (cfg (feature = "capture-spantrace")))] pub fn span_trace (& self) -> Option < & SpanTrace > { self . span_trace . as_ref () } pub (crate) fn format_backtrace < 'a > (& 'a self , trace : & 'a backtrace :: Backtrace ,) -> BacktraceFormatter < 'a > { BacktraceFormatter { filters : & self . filters , inner : trace , theme : self . theme , } } }
};
}
