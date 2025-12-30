// Generated macro for impl_format_escaped_plain (macro)
macro_rules! Depcrate_svgimpl_format_escaped_plain {
() => {
// Module: crate::svg
// Provides: {"impl_format_escaped_plain"}
// Dependencies: {}
macro_rules ! impl_format_escaped_plain { ($ ($ t : ty) ,*) => { $ (impl FormatEscaped for $ t { fn format_escaped (buf : & mut String , s : Self) { let _ = write ! (buf , "{}" , s) ; } }) * } ; }
};
}
