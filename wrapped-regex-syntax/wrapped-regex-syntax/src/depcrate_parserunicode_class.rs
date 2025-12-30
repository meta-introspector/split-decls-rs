// Generated macro for unicode_class (function)
macro_rules! Depcrate_parserunicode_class {
() => {
// Module: crate::parser
// Provides: {"unicode_class"}
// Dependencies: {}
fn unicode_class (name : & str) -> Option < CharClass > { UNICODE_CLASSES . binary_search_by (| & (s , _) | s . cmp (name)) . ok () . map (| i | { raw_class_to_expr (UNICODE_CLASSES [i] . 1) }) }
};
}
