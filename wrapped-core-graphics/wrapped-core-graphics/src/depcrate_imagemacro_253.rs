// Generated macro for macro_253 (macro)
macro_rules! Depcrate_imagemacro_253 {
() => {
// Module: crate::image
// Provides: {"macro_253"}
// Dependencies: {}
foreign_type ! { # [doc (hidden)] pub unsafe type CGImage { type CType = crate :: sys :: CGImage ; fn drop = CGImageRelease ; fn clone = | p | CFRetain (p as * const _) as * mut _ ; } }
};
}
