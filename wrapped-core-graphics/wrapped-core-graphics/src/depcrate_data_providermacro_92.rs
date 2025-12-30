// Generated macro for macro_92 (macro)
macro_rules! Depcrate_data_providermacro_92 {
() => {
// Module: crate::data_provider
// Provides: {"macro_92"}
// Dependencies: {}
foreign_type ! { # [doc (hidden)] pub unsafe type CGDataProvider { type CType = crate :: sys :: CGDataProvider ; fn drop = | cs | CFRelease (cs as * mut _) ; fn clone = | p | CFRetain (p as * const _) as * mut _ ; } }
};
}
