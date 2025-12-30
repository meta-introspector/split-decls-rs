// Generated macro for impl_527 (impl)
macro_rules! Depcrate_internalimpl_527 {
() => {
// Module: crate::internal
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SymbolInternStringLiteral { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx rustc_hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (path , [arg]) = expr . kind && let hir :: ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: SymbolIntern , def_id) && let hir :: ExprKind :: Lit (kind) = arg . kind && let rustc_ast :: LitKind :: Str (_ , _) = kind . node { cx . emit_span_lint (SYMBOL_INTERN_STRING_LITERAL , kind . span , SymbolInternStringLiteralDiag ,) ; } } }
};
}
