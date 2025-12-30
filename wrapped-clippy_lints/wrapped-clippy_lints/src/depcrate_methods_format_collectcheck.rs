// Generated macro for check (function)
macro_rules! Depcrate_methods_format_collectcheck {
() => {
// Module: crate::methods::format_collect
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , map_arg : & Expr < '_ > , map_span : Span) { if cx . typeck_results () . expr_ty (expr) . is_lang_item (cx , LangItem :: String) && let ExprKind :: Closure (closure) = map_arg . kind && let body = cx . tcx . hir_body (closure . body) && let Some (value) = peel_non_expn_blocks (body . value) && let Some (mac) = root_macro_call_first_node (cx , value) && is_format_macro (cx , mac . def_id) { span_lint_and_then (cx , FORMAT_COLLECT , expr . span , "use of `format!` to build up a string from an iterator" , | diag | { diag . span_help (map_span , "call `fold` instead") . span_help (value . span . source_callsite () , "... and use the `write!` macro here") . note ("this can be written more efficiently by appending to a `String` directly") ; } ,) ; } }
};
}
