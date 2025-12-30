// Generated macro for raw_class_to_expr (function)
macro_rules! Depcrate_parserraw_class_to_expr {
() => {
// Module: crate::parser
// Provides: {"raw_class_to_expr"}
// Dependencies: {}
fn raw_class_to_expr (raw : & [(char , char)]) -> CharClass { let range = | & (s , e) | ClassRange { start : s , end : e } ; CharClass :: new (raw . iter () . map (range) . collect ()) }
};
}
