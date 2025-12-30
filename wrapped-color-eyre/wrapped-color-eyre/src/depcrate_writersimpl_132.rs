// Generated macro for impl_132 (impl)
macro_rules! Depcrate_writersimpl_132 {
() => {
// Module: crate::writers
// Provides: {"impl_132"}
// Dependencies: {}
# [cfg (feature = "capture-spantrace")] impl fmt :: Display for FormattedSpanTrace < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use indenter :: indented ; use indenter :: Format ; if self . 0 . status () == SpanTraceStatus :: CAPTURED { write ! (indented (f) . with_format (Format :: Uniform { indentation : "  " }) , "{}" , color_spantrace :: colorize (self . 0)) ? ; } Ok (()) } }
};
}
