// Generated macro for make_sugg (function)
macro_rules! Depcrate_redundant_elsemake_sugg {
() => {
// Module: crate::redundant_else
// Provides: {"make_sugg"}
// Dependencies: {}
fn make_sugg (cx : & EarlyContext < '_ > , els_span : Span , default : & str , indent_relative_to : Option < Span >) -> String { let extracted = extract_else_block (& snippet (cx , els_span , default)) ; let indent = indent_relative_to . and_then (| s | indent_of (cx , s)) ; reindent_multiline (& extracted , false , indent) }
};
}
