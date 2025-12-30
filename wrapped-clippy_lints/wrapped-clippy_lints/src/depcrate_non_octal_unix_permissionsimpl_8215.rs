// Generated macro for impl_8215 (impl)
macro_rules! Depcrate_non_octal_unix_permissionsimpl_8215 {
() => {
// Module: crate::non_octal_unix_permissions
// Provides: {"impl_8215"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NonOctalUnixPermissions { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { match & expr . kind { ExprKind :: MethodCall (path , func , [param] , _) => { if let Some (adt) = cx . typeck_results () . expr_ty (func) . peel_refs () . ty_adt_def () && matches ! ((cx . tcx . get_diagnostic_name (adt . did ()) , path . ident . name) , (Some (sym :: FsOpenOptions | sym :: DirBuilder) , sym :: mode) | (Some (sym :: FsPermissions) , sym :: set_mode)) && let ExprKind :: Lit (_) = param . kind && param . span . eq_ctxt (expr . span) && param . span . check_source_text (cx , | src | ! matches ! (src . as_bytes () , [b'0' , b'o' | b'b' , ..])) { show_error (cx , param) ; } } , ExprKind :: Call (func , [param]) => { if let ExprKind :: Path (ref path) = func . kind && let Some (def_id) = cx . qpath_res (path , func . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: permissions_from_mode , def_id) && let ExprKind :: Lit (_) = param . kind && param . span . eq_ctxt (expr . span) && param . span . check_source_text (cx , | src | ! matches ! (src . as_bytes () , [b'0' , b'o' | b'b' , ..])) { show_error (cx , param) ; } } , _ => { } , } } }
};
}
