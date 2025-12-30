// Generated macro for impl_5076 (impl)
macro_rules! Depcrate_mem_replaceimpl_5076 {
() => {
// Module: crate::mem_replace
// Provides: {"impl_5076"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MemReplace { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Call (func , [dest , src]) = expr . kind && let ExprKind :: Path (ref func_qpath) = func . kind && let Some (def_id) = cx . qpath_res (func_qpath , func . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: mem_replace , def_id) && ! check_replace_option_with_none (cx , src , dest , expr . span) && ! check_replace_option_with_some (cx , src , dest , expr . span , self . msrv) && ! check_replace_with_default (cx , src , dest , expr , self . msrv) { check_replace_with_uninit (cx , src , dest , expr . span) ; } } }
};
}
