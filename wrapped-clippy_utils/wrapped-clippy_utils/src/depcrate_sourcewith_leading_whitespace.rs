// Generated macro for with_leading_whitespace (function)
macro_rules! Depcrate_sourcewith_leading_whitespace {
() => {
// Module: crate::source
// Provides: {"with_leading_whitespace"}
// Dependencies: {}
fn with_leading_whitespace (sm : & SourceMap , sp : Range < BytePos >) -> Range < BytePos > { map_range (sm , sp . clone () , | sf , src , range | { Some (with_leading_whitespace_inner (sf . lines () , src , range . clone ()) ? .. range . end) }) . unwrap_or (sp) }
};
}
