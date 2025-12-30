// Generated macro for with_source_text (function)
macro_rules! Depcrate_sourcewith_source_text {
() => {
// Module: crate::source
// Provides: {"with_source_text"}
// Dependencies: {}
fn with_source_text < T > (sm : & SourceMap , sp : Range < BytePos > , f : impl for < 'a > FnOnce (& 'a str) -> T) -> Option < T > { if let Some (src) = get_source_range (sm , sp) && let Some (src) = src . as_str () { Some (f (src)) } else { None } }
};
}
