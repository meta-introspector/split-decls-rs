// Generated macro for is_ok_wrapping (function)
macro_rules! Depcrate_methods_manual_ok_oris_ok_wrapping {
() => {
// Module: crate::methods::manual_ok_or
// Provides: {"is_ok_wrapping"}
// Dependencies: {}
fn is_ok_wrapping (cx : & LateContext < '_ > , map_expr : & Expr < '_ >) -> bool { match map_expr . kind { ExprKind :: Path (ref qpath) if cx . qpath_res (qpath , map_expr . hir_id) . ctor_parent (cx) . is_lang_item (cx , ResultOk) => { true } , ExprKind :: Closure (closure) => { let body = cx . tcx . hir_body (closure . body) ; if let PatKind :: Binding (_ , param_id , ..) = body . params [0] . pat . kind && let ExprKind :: Call (callee , [ok_arg]) = body . value . kind && callee . res (cx) . ctor_parent (cx) . is_lang_item (cx , ResultOk) { ok_arg . res_local_id () == Some (param_id) } else { false } } , _ => false , } }
};
}
