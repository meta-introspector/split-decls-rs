// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_create_dirimpl_1291 {
() => {
// Module: crate::create_dir
// Provides: {"impl_1291"}
// Dependencies: {}
impl LateLintPass < '_ > for CreateDir { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Call (func , [_]) = expr . kind && let ExprKind :: Path (ref path) = func . kind && let Some (def_id) = cx . qpath_res (path , func . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: fs_create_dir , def_id) && let QPath :: Resolved (_ , path) = path && let Some (last) = path . segments . last () { span_lint_and_then (cx , CREATE_DIR , expr . span , "calling `std::fs::create_dir` where there may be a better way" , | diag | { let mut suggestions = vec ! [(last . ident . span . shrink_to_hi () , "_all" . to_owned ())] ; if path . segments . len () == 1 { suggestions . push ((path . span . shrink_to_lo () , "std::fs::" . to_owned ())) ; } diag . multipart_suggestion_verbose ("consider calling `std::fs::create_dir_all` instead" , suggestions , Applicability :: MaybeIncorrect ,) ; } ,) ; } } }
};
}
