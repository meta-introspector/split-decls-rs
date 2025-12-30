// Generated macro for impl_134 (impl)
macro_rules! Depcrate_writersimpl_134 {
() => {
// Module: crate::writers
// Provides: {"impl_134"}
// Dependencies: {}
impl fmt :: Display for EnvSection < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let v = if std :: thread :: panicking () { panic_verbosity () } else { lib_verbosity () } ; write ! (f , "{}" , BacktraceOmited (! self . bt_captured)) ? ; let mut separated = HeaderWriter { inner : & mut * f , header : & "\n" , started : false , } ; write ! (& mut separated . ready () , "{}" , SourceSnippets (v)) ? ; # [cfg (feature = "capture-spantrace")] write ! (& mut separated . ready () , "{}" , SpanTraceOmited (self . span_trace)) ? ; Ok (()) } }
};
}
