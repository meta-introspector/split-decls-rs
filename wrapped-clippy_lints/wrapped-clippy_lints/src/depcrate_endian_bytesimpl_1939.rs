// Generated macro for impl_1939 (impl)
macro_rules! Depcrate_endian_bytesimpl_1939 {
() => {
// Module: crate::endian_bytes
// Provides: {"impl_1939"}
// Dependencies: {}
impl LateLintPass < '_ > for EndianBytes { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { let (prefix , name , ty_expr) = match expr . kind { ExprKind :: MethodCall (method_name , receiver , [] , ..) => (Prefix :: To , method_name . ident . name , receiver) , ExprKind :: Call (function , ..) if let ExprKind :: Path (qpath) = function . kind && let Some (def_id) = cx . qpath_res (& qpath , function . hir_id) . opt_def_id () && let Some (function_name) = cx . get_def_path (def_id) . last () => { (Prefix :: From , * function_name , expr) } , _ => return , } ; if ! expr . span . in_external_macro (cx . sess () . source_map ()) && let ty = cx . typeck_results () . expr_ty (ty_expr) && ty . is_primitive_ty () { maybe_lint_endian_bytes (cx , expr , prefix , name , ty) ; } } }
};
}
