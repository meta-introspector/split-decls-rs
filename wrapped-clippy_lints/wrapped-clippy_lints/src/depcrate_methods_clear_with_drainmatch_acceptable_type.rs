// Generated macro for match_acceptable_type (function)
macro_rules! Depcrate_methods_clear_with_drainmatch_acceptable_type {
() => {
// Module: crate::methods::clear_with_drain
// Provides: {"match_acceptable_type"}
// Dependencies: {}
fn match_acceptable_type (cx : & LateContext < '_ > , expr : & Expr < '_ > , types : & [rustc_span :: Symbol]) -> bool { let expr_ty = cx . typeck_results () . expr_ty (expr) . peel_refs () ; types . iter () . any (| & ty | expr_ty . is_diag_item (cx , ty)) || expr_ty . is_lang_item (cx , LangItem :: String) }
};
}
