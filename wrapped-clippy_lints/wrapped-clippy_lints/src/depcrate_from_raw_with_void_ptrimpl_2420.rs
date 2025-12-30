// Generated macro for impl_2420 (impl)
macro_rules! Depcrate_from_raw_with_void_ptrimpl_2420 {
() => {
// Module: crate::from_raw_with_void_ptr
// Provides: {"impl_2420"}
// Dependencies: {}
impl LateLintPass < '_ > for FromRawWithVoidPtr { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Call (box_from_raw , [arg]) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , seg)) = box_from_raw . kind && seg . ident . name == sym :: from_raw && let Some (type_str) = ty . basic_res () . opt_def_id () . and_then (| id | def_id_matches_type (cx , id)) && let arg_kind = cx . typeck_results () . expr_ty (arg) . kind () && let ty :: RawPtr (ty , _) = arg_kind && is_c_void (cx , * ty) { let msg = format ! ("creating a `{type_str}` from a void raw pointer") ; span_lint_and_help (cx , FROM_RAW_WITH_VOID_PTR , expr . span , msg , Some (arg . span) , "cast this to a pointer of the appropriate type" ,) ; } } }
};
}
