// Generated macro for impl_136 (impl)
macro_rules! Depcrate_writersimpl_136 {
() => {
// Module: crate::writers
// Provides: {"impl_136"}
// Dependencies: {}
# [cfg (feature = "capture-spantrace")] impl fmt :: Display for SpanTraceOmited < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (span_trace) = self . 0 { if span_trace . status () == SpanTraceStatus :: UNSUPPORTED { writeln ! (f , "Warning: SpanTrace capture is Unsupported.") ? ; write ! (f , "Ensure that you've setup a tracing-error ErrorLayer and the semver versions are compatible") ? ; } } Ok (()) } }
};
}
