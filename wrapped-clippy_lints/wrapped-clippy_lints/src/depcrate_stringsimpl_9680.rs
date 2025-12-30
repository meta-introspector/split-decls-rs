// Generated macro for impl_9680 (impl)
macro_rules! Depcrate_stringsimpl_9680 {
() => {
// Module: crate::strings
// Provides: {"impl_9680"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for StringAdd { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if e . span . in_external_macro (cx . sess () . source_map ()) { return ; } match e . kind { ExprKind :: Binary (Spanned { node : BinOpKind :: Add , .. } , left , _ ,) => { if is_string (cx , left) { if ! is_lint_allowed (cx , STRING_ADD_ASSIGN , e . hir_id) { let parent = get_parent_expr (cx , e) ; if let Some (p) = parent && let ExprKind :: Assign (target , _ , _) = p . kind && SpanlessEq :: new (cx) . eq_expr (target , left) { return ; } } span_lint (cx , STRING_ADD , e . span , "you added something to a string. Consider using `String::push_str()` instead" ,) ; } } , ExprKind :: Assign (target , src , _) => { if is_string (cx , target) && is_add (cx , src , target) { span_lint (cx , STRING_ADD_ASSIGN , e . span , "you assigned the result of adding something to this string. Consider using \
                         `String::push_str()` instead" ,) ; } } , ExprKind :: Index (target , _idx , _) => { let e_ty = cx . typeck_results () . expr_ty_adjusted (target) . peel_refs () ; if e_ty . is_str () || is_type_lang_item (cx , e_ty , LangItem :: String) { span_lint (cx , STRING_SLICE , e . span , "indexing into a string may panic if the index is within a UTF-8 character" ,) ; } } , _ => { } , } } }
};
}
