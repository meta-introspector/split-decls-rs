// Generated macro for ascii_class (function)
macro_rules! Depcrate_parserascii_class {
() => {
// Module: crate::parser
// Provides: {"ascii_class"}
// Dependencies: {}
fn ascii_class (name : & str) -> Option < CharClass > { ASCII_CLASSES . binary_search_by (| & (s , _) | s . cmp (name)) . ok () . map (| i | { raw_class_to_expr (ASCII_CLASSES [i] . 1) }) }
};
}
