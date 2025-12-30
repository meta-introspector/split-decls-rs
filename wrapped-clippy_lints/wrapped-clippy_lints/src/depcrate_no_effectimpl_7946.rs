// Generated macro for impl_7946 (impl)
macro_rules! Depcrate_no_effectimpl_7946 {
() => {
// Module: crate::no_effect
// Provides: {"impl_7946"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NoEffect { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if self . check_no_effect (cx , stmt) { return ; } check_unnecessary_operation (cx , stmt) ; } fn check_block (& mut self , _ : & LateContext < 'tcx > , _ : & 'tcx rustc_hir :: Block < 'tcx >) { self . local_bindings . push (Vec :: default ()) ; } fn check_block_post (& mut self , cx : & LateContext < 'tcx > , _ : & 'tcx rustc_hir :: Block < 'tcx >) { for hir_id in self . local_bindings . pop () . unwrap () { if let Some (span) = self . underscore_bindings . swap_remove (& hir_id) { span_lint_hir (cx , NO_EFFECT_UNDERSCORE_BINDING , hir_id , span , "binding to `_` prefixed variable with no side-effect" ,) ; } } } fn check_expr (& mut self , _ : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx >) { if let Some (def_id) = path_to_local (expr) { self . underscore_bindings . swap_remove (& def_id) ; } } }
};
}
