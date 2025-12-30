// Generated macro for macro_209 (macro)
macro_rules! Depcrate_event_sourcemacro_209 {
() => {
// Module: crate::event_source
// Provides: {"macro_209"}
// Dependencies: {}
foreign_type ! { # [doc (hidden)] pub unsafe type CGEventSource { type CType = crate :: sys :: CGEventSource ; fn drop = | p | CFRelease (p as * mut _) ; fn clone = | p | CFRetain (p as * const _) as * mut _ ; } }
};
}
