// Generated macro for impl_9706 (impl)
macro_rules! Depcrate_strlen_on_c_stringsimpl_9706 {
() => {
// Module: crate::strlen_on_c_strings
// Provides: {"impl_9706"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for StrlenOnCStrings { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if ! expr . span . from_expansion () && let ExprKind :: Call (func , [recv]) = expr . kind && let ExprKind :: Path (path) = & func . kind && let Some (did) = cx . qpath_res (path , func . hir_id) . opt_def_id () && match_libc_symbol (cx , did , sym :: strlen) && let ExprKind :: MethodCall (path , self_arg , [] , _) = recv . kind && ! recv . span . from_expansion () && path . ident . name == sym :: as_ptr { let ctxt = expr . span . ctxt () ; let span = match cx . tcx . parent_hir_node (expr . hir_id) { Node :: Block (& Block { rules : BlockCheckMode :: UnsafeBlock (UnsafeSource :: UserProvided) , span , .. }) if span . ctxt () == ctxt && ! is_expr_unsafe (cx , self_arg) => span , _ => expr . span , } ; let ty = cx . typeck_results () . expr_ty (self_arg) . peel_refs () ; let mut app = Applicability :: MachineApplicable ; let val_name = snippet_with_context (cx , self_arg . span , ctxt , ".." , & mut app) . 0 ; let method_name = if is_type_diagnostic_item (cx , ty , sym :: cstring_type) { "as_bytes" } else if is_type_lang_item (cx , ty , LangItem :: CStr) { "to_bytes" } else { return ; } ; span_lint_and_sugg (cx , STRLEN_ON_C_STRINGS , span , "using `libc::strlen` on a `CString` or `CStr` value" , "try" , format ! ("{val_name}.{method_name}().len()") , app ,) ; } } }
};
}
