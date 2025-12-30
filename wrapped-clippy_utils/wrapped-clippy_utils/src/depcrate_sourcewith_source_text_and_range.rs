// Generated macro for with_source_text_and_range (function)
macro_rules! Depcrate_sourcewith_source_text_and_range {
() => {
// Module: crate::source
// Provides: {"with_source_text_and_range"}
// Dependencies: {}
fn with_source_text_and_range < T > (sm : & SourceMap , sp : Range < BytePos > , f : impl for < 'a > FnOnce (& 'a str , Range < usize >) -> T ,) -> Option < T > { if let Some (src) = get_source_range (sm , sp) && let Some (text) = & src . sf . src { Some (f (text , src . range)) } else { None } }
};
}
