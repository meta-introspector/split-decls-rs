// Generated macro for PanicReport (struct)
macro_rules! Depcrate_configPanicReport {
() => {
// Module: crate::config
// Provides: {"PanicReport"}
// Dependencies: {}
# [doc = " A type representing an error report for a panic."] pub struct PanicReport < 'a > { hook : & 'a PanicHook , panic_info : & 'a std :: panic :: PanicInfo < 'a > , backtrace : Option < backtrace :: Backtrace > , # [cfg (feature = "capture-spantrace")] span_trace : Option < tracing_error :: SpanTrace > , }
};
}
