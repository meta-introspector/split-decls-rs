// Generated macro for span_from_toml_range (function)
macro_rules! Depcrate_confspan_from_toml_range {
() => {
// Module: crate::conf
// Provides: {"span_from_toml_range"}
// Dependencies: {}
fn span_from_toml_range (file : & SourceFile , span : Range < usize >) -> Span { Span :: new (file . start_pos + BytePos :: from_usize (span . start) , file . start_pos + BytePos :: from_usize (span . end) , SyntaxContext :: root () , None ,) }
};
}
