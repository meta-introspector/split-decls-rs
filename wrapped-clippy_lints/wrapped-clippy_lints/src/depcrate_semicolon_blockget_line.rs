// Generated macro for get_line (function)
macro_rules! Depcrate_semicolon_blockget_line {
() => {
// Module: crate::semicolon_block
// Provides: {"get_line"}
// Dependencies: {}
fn get_line (cx : & LateContext < '_ > , span : Span) -> Option < usize > { cx . sess () . source_map () . lookup_line (span . lo ()) . ok () . map (| line | line . line) }
};
}
