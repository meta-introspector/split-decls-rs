// Generated macro for is_string (function)
macro_rules! Depcrate_format_push_stringis_string {
() => {
// Module: crate::format_push_string
// Provides: {"is_string"}
// Dependencies: {}
fn is_string (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (e) . peel_refs () . is_lang_item (cx , LangItem :: String) }
};
}
