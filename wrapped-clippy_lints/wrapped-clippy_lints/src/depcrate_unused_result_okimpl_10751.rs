// Generated macro for impl_10751 (impl)
macro_rules! Depcrate_unused_result_okimpl_10751 {
() => {
// Module: crate::unused_result_ok
// Provides: {"impl_10751"}
// Dependencies: {}
impl LateLintPass < '_ > for UnusedResultOk { fn check_stmt (& mut self , cx : & LateContext < '_ > , stmt : & Stmt < '_ >) { if let StmtKind :: Semi (expr) = stmt . kind && let ExprKind :: MethodCall (ok_path , recv , [] , ..) = expr . kind && ok_path . ident . name == sym :: ok && is_type_diagnostic_item (cx , cx . typeck_results () . expr_ty (recv) , sym :: Result) && ! stmt . span . in_external_macro (cx . sess () . source_map ()) { let ctxt = expr . span . ctxt () ; let mut applicability = Applicability :: MaybeIncorrect ; let snippet = snippet_with_context (cx , recv . span , ctxt , "" , & mut applicability) . 0 ; let sugg = format ! ("let _ = {snippet}") ; span_lint_and_sugg (cx , UNUSED_RESULT_OK , expr . span , "ignoring a result with `.ok()` is misleading" , "consider using `let _ =` and removing the call to `.ok()` instead" , sugg , applicability ,) ; } } }
};
}
