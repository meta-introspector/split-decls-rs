// Generated macro for check (function)
macro_rules! Depcrate_methods_no_effect_replacecheck {
() => {
// Module: crate::methods::no_effect_replace
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx rustc_hir :: Expr < '_ > , arg1 : & 'tcx rustc_hir :: Expr < '_ > , arg2 : & 'tcx rustc_hir :: Expr < '_ > ,) { let ty = cx . typeck_results () . expr_ty (expr) . peel_refs () ; if ! (ty . is_str () || ty . is_lang_item (cx , LangItem :: String)) { return ; } if let ExprKind :: Lit (spanned) = & arg1 . kind && let Some (param1) = lit_string_value (& spanned . node) && let ExprKind :: Lit (spanned) = & arg2 . kind && let LitKind :: Str (param2 , _) = & spanned . node && param1 == param2 . as_str () { span_lint (cx , NO_EFFECT_REPLACE , expr . span , "replacing text with itself") ; return ; } if SpanlessEq :: new (cx) . eq_expr (arg1 , arg2) { span_lint (cx , NO_EFFECT_REPLACE , expr . span , "replacing text with itself") ; } }
};
}
