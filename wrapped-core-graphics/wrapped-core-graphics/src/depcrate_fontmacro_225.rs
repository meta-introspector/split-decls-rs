// Generated macro for macro_225 (macro)
macro_rules! Depcrate_fontmacro_225 {
() => {
// Module: crate::font
// Provides: {"macro_225"}
// Dependencies: {}
foreign_type ! { # [doc (hidden)] pub unsafe type CGFont : Send + Sync { type CType = crate :: sys :: CGFont ; fn drop = | p | CFRelease (p as * mut _) ; fn clone = | p | CFRetain (p as * const _) as * mut _ ; } }
};
}
