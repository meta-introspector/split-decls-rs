// Generated macro for impl_147 (impl)
macro_rules! Depcrate_fallbackimpl_147 {
() => {
// Module: crate::fallback
// Provides: {"impl_147"}
// Dependencies: {}
impl Debug for Span { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { # [cfg (span_locations)] return write ! (f , "bytes({}..{})" , self . lo , self . hi) ; # [cfg (not (span_locations))] write ! (f , "Span") } }
};
}
