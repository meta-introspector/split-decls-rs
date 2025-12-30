// Generated macro for macro_202 (macro)
macro_rules! Depcrate_eventmacro_202 {
() => {
// Module: crate::event
// Provides: {"macro_202"}
// Dependencies: {}
foreign_type ! { # [doc (hidden)] pub unsafe type CGEvent { type CType = crate :: sys :: CGEvent ; fn drop = | p | CFRelease (p as * mut _) ; fn clone = | p | CFRetain (p as * const _) as * mut _ ; } }
};
}
